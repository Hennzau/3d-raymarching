use std::f32::consts::FRAC_PI_2;

use glam::Vec3;

use winit::{
    event::{
        ElementState,
        KeyEvent,
        MouseButton
    },
    keyboard::{
        KeyCode,
        PhysicalKey
    },
    window::{
        CursorGrabMode,
        Window
    }
};

use crate::logic::camera::{
    Camera,
    CameraController
};

#[derive(PartialEq)]
pub enum PlayState {
    Playing,
    Pause,
}

#[derive(PartialEq)]
pub enum PipelineType {
    TestRasterizer,
    TestRayMarcher,
}

pub struct Play {
    pub camera: Camera,
    pub controller: CameraController,

    pub state: PlayState,
    pub pipeline: PipelineType,
}

impl Play {
    pub fn new() -> Self {
        return Self {
            camera: Camera::new(),
            controller: CameraController::new(),

            state: PlayState::Pause,
            pipeline: PipelineType::TestRasterizer,
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
                            self.state = PlayState::Pause;
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyE) => {
                        if state == ElementState::Pressed {
                            self.pipeline = PipelineType::TestRasterizer;
                            self.camera.position = Vec3::new(0f32, -3f32, 0f32);
                            self.camera.rotation = Vec3::new(FRAC_PI_2, 0f32, 0f32);
                        }
                    }
                    PhysicalKey::Code(KeyCode::KeyR) => {
                        if state == ElementState::Pressed {
                            self.pipeline = PipelineType::TestRayMarcher;
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

    #[cfg(target_os = "windows")]
    fn grab_cursor(window: &Window) {
        window.set_cursor_grab(CursorGrabMode::Confined).expect("Failed to set cursor grab mode");
    }

    #[cfg(target_os = "macos")]
    fn grab_cursor(window: &Window) {
        window.set_cursor_grab(CursorGrabMode::Locked).expect("Failed to set cursor grab mode");
    }

    pub fn process_mouse_input(&mut self, window: &Window, state: ElementState, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => {
                if state == ElementState::Pressed {
                    Self::grab_cursor(window);
                    window.set_cursor_visible(false);
                    self.state = PlayState::Playing;
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
        if self.state == PlayState::Playing {
            self.controller.process_mouse_motion(delta);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state == PlayState::Playing {
            self.controller.update(delta_time, &mut self.camera);
        }
    }
}

