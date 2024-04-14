use std::{
    borrow::Cow,
    f32::consts::PI,
    mem,
};

use wgpu::{
    Device,
    Surface,
    Adapter,
    RenderPipeline,
    Queue,
    SurfaceConfiguration,
    VertexBufferLayout,
    BindGroup,
    Buffer,
    util::DeviceExt,
};

use bytemuck::{
    Pod,
    Zeroable,
};

use glam::{
    Mat4,
    Vec3,
};

pub mod terrain;
pub mod chunk;

pub struct VoxRenderer {
    bind_group: BindGroup,
    render_pipeline: RenderPipeline,

    vertex_buf: Buffer,
    index_buf: Buffer,
    index_count: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 4],
}

fn vertex(pos: [i8; 3]) -> Vertex {
    Vertex {
        _pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0],
    }
}

fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
    let vertex_data = [
        // top (0, 0, 1)
        vertex([-1, -1, 1]),
        vertex([1, -1, 1]),
        vertex([1, 1, 1]),
        vertex([-1, 1, 1]),
        // bottom (0, 0, -1)
        vertex([-1, 1, -1]),
        vertex([1, 1, -1]),
        vertex([1, -1, -1]),
        vertex([-1, -1, -1]),
        // right (1, 0, 0)
        vertex([1, -1, -1]),
        vertex([1, 1, -1]),
        vertex([1, 1, 1]),
        vertex([1, -1, 1]),
        // left (-1, 0, 0)
        vertex([-1, -1, 1]),
        vertex([-1, 1, 1]),
        vertex([-1, 1, -1]),
        vertex([-1, -1, -1]),
        // front (0, 1, 0)
        vertex([1, 1, -1]),
        vertex([-1, 1, -1]),
        vertex([-1, 1, 1]),
        vertex([1, 1, 1]),
        // back (0, -1, 0)
        vertex([1, -1, 1]),
        vertex([-1, -1, 1]),
        vertex([-1, -1, -1]),
        vertex([1, -1, -1]),
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data.to_vec(), index_data.to_vec())
}


pub struct Cube {
    vertex_buf: Buffer,
    index_buf: Buffer,
    buffer_layout: VertexBufferLayout<'static>,
    transform_buf: Buffer,
    index_count: usize,
}

impl Cube {
    pub fn new(device: &Device) -> Self {
        let vertex_size = mem::size_of::<Vertex>();
        let (vertex_data, index_data) = create_vertices();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let transform = Mat4::from_translation(Vec3::new(0f32, 3f32, 0f32));
        let transform_ref: &[f32; 16] = transform.as_ref();
        let transform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(transform_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: vertex_size as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        };

        return Self {
            vertex_buf,
            index_buf,
            buffer_layout,
            index_count: index_data.len(),
            transform_buf,
        };
    }
}


impl VoxRenderer {
    pub fn new(device: &Device, config: &SurfaceConfiguration, surface: &Surface, adapter: &Adapter) -> Self {
        let projection = glam::Mat4::perspective_rh(70f32 * PI / 180f32, config.width as f32 / config.height as f32, 0.01, 100.0);
        let view = glam::Mat4::look_at_rh(
            glam::Vec3::new(1.5f32, -5.0, 3.0),
            glam::Vec3::ZERO,
            glam::Vec3::Z,
        );

        let projection_view = projection * view;
        let projection_view_ref: &[f32; 16] = projection_view.as_ref();
        let projection_view_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Projection View Buffer"),
            contents: bytemuck::cast_slice(projection_view_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cube = Cube::new(&device);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_view_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: cube.transform_buf.as_entire_binding(),
                },
            ],
            label: None,
        });

        // Load the shaders from disk
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("renderer/terrain.wgsl"))),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[cube.buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        return Self {
            bind_group,
            render_pipeline,
            vertex_buf: cube.vertex_buf,
            index_buf: cube.index_buf,
            index_count: cube.index_count,
        };
    }

    pub fn render(&self, device: &Device, surface: &Surface, queue: &Queue) {
        let frame = surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: None,
            });
        {
            let mut rpass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }
}