use winit::event::KeyEvent;

use crate::vox::{
    camera::{
        Camera,
        CameraController,
    },
};
use crate::vox::world::World;

pub mod camera;

pub mod world;
pub mod chunk;
pub mod cube;

pub struct VoxLogic {
    pub camera: Camera,
    controller: CameraController,
    pub world: World,
}

impl VoxLogic {
    pub fn new(aspect_ratio: f32) -> Self {
        return Self {
            camera: Camera::new(aspect_ratio),
            controller: CameraController::new(50f32),
            world: World::new(),
        };
    }

    pub fn process_keyboard(&mut self, event: KeyEvent) {
        self.controller.process(event.clone());
    }

    pub fn process_resize(&mut self, new_size: (u32, u32)) {
        self.camera.process_resize(new_size.0 as f32 / new_size.1 as f32);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.controller.update(delta_time, &mut self.camera);
    }
}