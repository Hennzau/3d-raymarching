use crate::logic::Logic;
use crate::WGPUBackend;

pub struct Renderer {}

impl Renderer {
    pub fn new(wgpu_backend: &WGPUBackend, logic: &Logic) -> Self {
        return Self {};
    }

    pub fn update(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {}

    pub fn process_resize(&mut self, wgpu_backend: &WGPUBackend, logic: &Logic) {}

    pub fn render(&self, wgpu_backend: &WGPUBackend, logic: &Logic) {}
}