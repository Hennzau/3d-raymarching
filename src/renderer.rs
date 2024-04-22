use wgpu::{
    LoadOp,
    Operations,
    RenderPassColorAttachment,
    RenderPassDescriptor
};
use wgpu::util::DeviceExt;

use crate::{
    logic::Logic,
    WGPUBackend
};
use crate::renderer::pipeline::ColorVertex;

pub mod pipeline;

pub struct Renderer {
    pipeline: pipeline::ColorPipeline,

    projection_view_model_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Renderer {
    pub fn new(wgpu_backend: &WGPUBackend, logic: &Logic) -> Self {
        let pipeline = pipeline::ColorPipeline::new(wgpu_backend);

        let projection_view_model_data = logic.camera.build_projection_view_matrix(wgpu_backend.config.width as f32 / wgpu_backend.config.height as f32);
        let projection_view_model_ref: &[f32; 16] = projection_view_model_data.as_ref();
        let projection_view_model_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(projection_view_model_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = wgpu_backend.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: projection_view_model_buffer.as_entire_binding(),
                },
            ],
        });

        // Back culled green cube
        let mut vertices = Vec::<ColorVertex>::new();
        vertices.push(ColorVertex { position: [0.0, 0.0, 0.0], color: [0, 255, 0, 255] });
        vertices.push(ColorVertex { position: [1.0, 0.0, 0.0], color: [0, 255, 0, 255] });
        vertices.push(ColorVertex { position: [0.0, 0.0, 1.0], color: [0, 255, 0, 255] });
        vertices.push(ColorVertex { position: [1.0, 0.0, 1.0], color: [0, 255, 0, 255] });

        let vertex_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let index_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        return Self {
            pipeline,
            projection_view_model_buffer,
            bind_group,
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {
        let projection_view_model_data = logic.camera.build_projection_view_matrix(wgpu_backend.config.width as f32 / wgpu_backend.config.height as f32);
        let projection_view_model_ref: &[f32; 16] = projection_view_model_data.as_ref();

        wgpu_backend.queue.write_buffer(&self.projection_view_model_buffer, 0, bytemuck::cast_slice(projection_view_model_ref));
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