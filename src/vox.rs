use crate::vox::renderer::VoxRenderer;

pub mod renderer;

pub struct VoxLogic {}

impl VoxLogic {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, renderer: &mut VoxRenderer) {}
}