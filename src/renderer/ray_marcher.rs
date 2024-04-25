use wgpu::util::DeviceExt;

use crate::{
    WGPUBackend,
    logic::play::Play,
    renderer::{
        pipeline,
        pipeline::SimpleVertex
    }
};


pub struct TestRayMarcher {
    pipeline: pipeline::RayMarchingPipeline,

    camera_position_buffer: wgpu::Buffer,
    camera_inverted_projection_buffer: wgpu::Buffer,
    camera_inverted_view_buffer: wgpu::Buffer,
    surface_configuration_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl TestRayMarcher {
    pub fn new(wgpu_backend: &WGPUBackend, play: &Play) -> Self {
        let pipeline = pipeline::RayMarchingPipeline::new(wgpu_backend);

        let camera_position_data = play.camera.position;
        let camera_position_ref: &[f32; 3] = camera_position_data.as_ref();
        let camera_position_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(camera_position_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_inverted_projection_data = play.camera.get_inverted_projection_matrix(wgpu_backend.config.width as f32 / wgpu_backend.config.height as f32);
        let camera_inverted_projection_ref: &[f32; 16] = camera_inverted_projection_data.as_ref();
        let camera_inverted_projection_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(camera_inverted_projection_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_inverted_view_data = play.camera.get_inverted_view_matrix();
        let camera_inverted_view_ref: &[f32; 16] = camera_inverted_view_data.as_ref();
        let camera_inverted_view_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(camera_inverted_view_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let surface_configuration_data = [wgpu_backend.config.width as f32, wgpu_backend.config.height as f32];
        let surface_configuration_buffer = wgpu_backend.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(surface_configuration_data.as_ref()),
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
                    resource: camera_inverted_projection_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: camera_inverted_view_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: surface_configuration_buffer.as_entire_binding(),
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
            camera_inverted_projection_buffer,
            camera_inverted_view_buffer,
            surface_configuration_buffer,

            bind_group,

            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        };
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let camera_position_data = play.camera.position;
        let camera_position_ref: &[f32; 3] = camera_position_data.as_ref();

        wgpu_backend.queue.write_buffer(&self.camera_position_buffer, 0, bytemuck::cast_slice(camera_position_ref));

        let camera_inverted_view_data = play.camera.get_inverted_view_matrix();
        let camera_inverted_view_ref: &[f32; 16] = camera_inverted_view_data.as_ref();

        wgpu_backend.queue.write_buffer(&self.camera_inverted_view_buffer, 0, bytemuck::cast_slice(camera_inverted_view_ref));
    }

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, play: &Play) {
        let surface_configuration_data = [wgpu_backend.config.width as f32, wgpu_backend.config.height as f32];
        wgpu_backend.queue.write_buffer(&self.surface_configuration_buffer, 0, bytemuck::cast_slice(surface_configuration_data.as_ref()));

        let camera_inverted_projection_data = play.camera.get_inverted_projection_matrix(wgpu_backend.config.width as f32 / wgpu_backend.config.height as f32);
        let camera_inverted_projection_ref: &[f32; 16] = camera_inverted_projection_data.as_ref();

        wgpu_backend.queue.write_buffer(&self.camera_inverted_projection_buffer, 0, bytemuck::cast_slice(camera_inverted_projection_ref));
    }

    pub fn render<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}