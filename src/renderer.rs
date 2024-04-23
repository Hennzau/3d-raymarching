use wgpu::{
    LoadOp,
    Operations,
    RenderPassColorAttachment,
    RenderPassDescriptor,
    util::DeviceExt
};

use crate::{
    logic::Logic,
    renderer::pipeline::SimpleVertex,
    WGPUBackend
};

pub mod pipeline;
pub mod rasterizer;
pub mod raymarcher;

pub struct Renderer {
    pipeline: pipeline::RayMarchingPipeline,

    camera_position_buffer: wgpu::Buffer,
    camera_direction_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Renderer {
    pub fn new(wgpu_backend: &WGPUBackend, logic: &Logic) -> Self {
        let pipeline = pipeline::RayMarchingPipeline::new(wgpu_backend);

        let camera_position_data = logic.camera.position;
        let camera_position_ref: &[f32; 3] = camera_position_data.as_ref();
        let camera_position_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(camera_position_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_direction_data = logic.camera.rotation;
        let camera_direction_ref: &[f32; 3] = camera_direction_data.as_ref();
        let camera_direction_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(camera_direction_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = wgpu_backend.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_position_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: camera_direction_buffer.as_entire_binding(),
                },
            ],
        });

        // Back culled green cube
        let mut vertices = Vec::<SimpleVertex>::new();
        vertices.push(SimpleVertex { position: [-1.0, 1.0] });
        vertices.push(SimpleVertex { position: [-1.0, -1.0] });
        vertices.push(SimpleVertex { position: [1.0, -1.0] });
        vertices.push(SimpleVertex { position: [1.0, 1.0] });

        let vertex_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

        let index_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            pipeline,
            camera_position_buffer,
            camera_direction_buffer,
            bind_group,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        let camera_position_data = logic.camera.position;
        let camera_position_ref: &[f32; 3] = camera_position_data.as_ref();

        let camera_direction_data = logic.camera.rotation;
        let camera_direction_ref: &[f32; 3] = camera_direction_data.as_ref();

        wgpu_backend.queue.write_buffer(&self.camera_position_buffer, 0, bytemuck::cast_slice(camera_position_ref));
        wgpu_backend.queue.write_buffer(&self.camera_direction_buffer, 0, bytemuck::cast_slice(camera_direction_ref));
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {}

    pub fn render(&self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        let frame = wgpu_backend.surface.get_current_texture().expect("Failed to acquire next swap chain texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = wgpu_backend.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);

            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        wgpu_backend.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}