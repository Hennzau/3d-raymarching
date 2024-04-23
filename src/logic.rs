use std::f32::consts::FRAC_PI_2;

use glam::Vec3;

use winit::{
    window::{
        Window,
        CursorGrabMode,
    },
    event::{
        ElementState,
        KeyEvent,
        MouseButton,
    },
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
};

use crate::logic::camera::{
    Camera,
    CameraController,
};

pub mod camera;

#[derive(PartialEq)]
pub enum State {
    Playing,
    Pause
}

#[derive(PartialEq)]
pub enum Scene {
    TestRasterizer,
    TestRayMarcher,
}

pub struct Logic {
    pub camera: Camera,
    pub controller: CameraController,

    pub state: State,
    pub scene: Scene,
}

impl Logic {
    pub fn new() -> Self {
        return Self {
            camera: Camera::new(),
            controller: CameraController::new(),
            state: State::Pause,
            scene: Scene::TestRayMarcher,
        };
    }

    pub fn process_keyboard(&mut self, window: &Window, key_event: KeyEvent) {
        match key_event {
            KeyEvent {
                physical_key,
                state,
                ..
            } => {
                match physical_key {
                    PhysicalKey::Code(KeyCode::Escape) => {
                        if state == ElementState::Pressed {
                            window.set_cursor_grab(CursorGrabMode::None).expect("Failed to set cursor grab mode");
                            window.set_cursor_visible(true);
                            self.state = State::Pause;
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyE) => {
                        if state == ElementState::Pressed {
                            self.scene = Scene::TestRasterizer;
                            self.camera.position = Vec3::new(0f32, -3f32, 0f32);
                            self.camera.rotation = Vec3::new(FRAC_PI_2, 0f32, 0f32);
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyR) => {
                        if state == ElementState::Pressed {
                            self.scene = Scene::TestRayMarcher;
                            self.camera.position = Vec3::new(0f32, -3f32, 0f32);
                            self.camera.rotation = Vec3::new(FRAC_PI_2, 0f32, 0f32);
                        }
                    }
                    PhysicalKey::Code(KeyCode::Enter) => {
                        self.camera.position = Vec3::new(0f32, -3f32, 0f32);
                        self.camera.rotation = Vec3::new(FRAC_PI_2, 0f32, 0f32);
                    }
                    _ => {}
                }
            }
        }

        self.controller.process_keyboard(key_event);
    }

    pub fn process_mouse_input(&mut self, window: &Window, state: ElementState, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => {
                if state == ElementState::Pressed {
                    window.set_cursor_grab(CursorGrabMode::Locked).expect("Failed to set cursor grab mode");
                    window.set_cursor_visible(false);
                    self.state = State::Playing;
                }
            }
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    pub fn process_mouse_motion(&mut self, delta: (f32, f32)) {
        if self.state == State::Playing {
            self.controller.process_mouse_motion(delta);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state == State::Playing {
            self.controller.update(delta_time, &mut self.camera);
        }
    }
}