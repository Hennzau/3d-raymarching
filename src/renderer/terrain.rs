use std::{
    borrow::Cow,
    mem,
};

use bytemuck::{
    Pod,
    Zeroable,
};

use glam::Mat4;

use wgpu::{
    Adapter,
    BindGroup,
    Buffer,
    Queue,
    Device,
    RenderPipeline,
    ShaderModule,
    Surface,
    Face,
    TextureFormat,
    util::DeviceExt,
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TerrainVertex {
    position: [f32; 3],
    color: [u8; 4],
}

impl TerrainVertex {
    pub fn new(position: [f32; 3], color: [u8; 4]) -> Self {
        return Self {
            position,
            color,
        };
    }
}

pub struct TerrainRenderer {
    pub bind_group: BindGroup,
    pub shader: ShaderModule,
    pub render_pipeline: RenderPipeline,

    projection_view_uniform: Buffer,
}

impl TerrainRenderer {
    pub fn new(device: &Device, surface: &Surface, adapter: &Adapter) -> Self {
        let projection_view_data = Mat4::IDENTITY;
        let projection_view_ref: &[f32; 16] = projection_view_data.as_ref();
        let projection_view_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(projection_view_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("BindGroupLayout for Terrain Renderer"),
            entries: &[
                wgpu::BindGroupLayoutEntry { // Projection * View Matrix
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("BindGroup for Terrain Renderer"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_view_uniform.as_entire_binding(),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("terrain.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let vertex_size = mem::size_of::<TerrainVertex>();

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Unorm8x4,
                    offset: 3 * 4,
                    shader_location: 1,
                }
            ],
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        return Self {
            bind_group,
            shader,
            render_pipeline,

            projection_view_uniform,
        };
    }

    pub fn update_projection_view_uniform(&mut self, queue: &Queue, projection_view_matrix: Mat4) {
        let projection_view_ref: &[f32; 16] = projection_view_matrix.as_ref();

        queue.write_buffer(&self.projection_view_uniform, 0, bytemuck::cast_slice(projection_view_ref));
    }
}