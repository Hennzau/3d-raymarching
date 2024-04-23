use crate::renderer::pipeline;
use crate::WGPUBackend;

pub struct TestRasterizer {
    pipeline: pipeline::ColorPipeline,

    projection_view_matrix: wgpu::Buffer,

    bind_group: wgpu::BindGroup,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl TestRasterizer {
    pub fn new (wgpu_backend: &WGPUBackend) -> Self {

    }
}