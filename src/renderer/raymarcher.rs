use crate::renderer::pipeline;
use crate::WGPUBackend;

pub struct TestRayMarcher {
    pipeline: pipeline::ColorPipeline,

    camera_position_buffer: wgpu::Buffer,
    camera_direction_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl TestRayMarcher {
    pub fn new (wgpu_backend: &WGPUBackend) -> Self {

    }
}