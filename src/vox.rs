use winit::event::KeyEvent;

use crate::vox::{
    camera::{
        Camera,
        CameraController,
    },
    renderer::VoxRenderer,
};

pub mod renderer;
pub mod camera;

pub struct VoxLogic {
    camera: Camera,
    controller: CameraController,
}

impl VoxLogic {
    pub fn new(aspect_ratio: f32) -> Self {
        return Self {
            camera: Camera::new(aspect_ratio),
            controller: CameraController::new(1f32),
        };
    }

    pub fn process_keyboard(&mut self, event: KeyEvent) {
        self.controller.process(event.clone());
    }

    pub fn process_resize(&mut self, new_size: (usize, usize)) {
        self.camera.process_resize(new_size.0 as f32 / new_size.1 as f32);
    }

    pub fn update(&mut self, renderer: &mut VoxRenderer) {
        self.controller.update(&mut self.camera);
    }
}