use wgpu::util::DeviceExt;

use crate::{
    WGPUBackend,
    logic::play::Play,
    renderer::{
        pipeline,
        pipeline::ColorVertex
    }
};

pub struct TestRasterizer {
    pipeline: pipeline::ColorPipeline,

    projection_view_model_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl TestRasterizer {
    pub fn new(wgpu_backend: &WGPUBackend, play: &Play) -> Self {
        let pipeline = pipeline::ColorPipeline::new(wgpu_backend);

        let projection_view_model_data = play.camera.build_projection_view_matrix(wgpu_backend.config.width as f32 / wgpu_backend.config.height as f32);
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

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let projection_view_model_data = play.camera.build_projection_view_matrix(wgpu_backend.config.width as f32 / wgpu_backend.config.height as f32);
        let projection_view_model_ref: &[f32; 16] = projection_view_model_data.as_ref();

        wgpu_backend.queue.write_buffer(&self.projection_view_model_buffer, 0, bytemuck::cast_slice(projection_view_model_ref));
    }
}