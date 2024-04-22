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

use crate::logic::camera::Camera;

pub mod camera;

pub struct Logic {
    camera: Camera,
}

impl Logic {
    pub fn new() -> Self {
        return Self {
            camera: Camera::new()
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
                        window.set_cursor_grab(CursorGrabMode::None).expect("TODO: panic message");
                        window.set_cursor_visible(true);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn process_mouse_input(&mut self, window: &Window, state: ElementState, mouse_button: MouseButton) {
        match mouse_button {
            MouseButton::Left => {
                window.set_cursor_grab(CursorGrabMode::Confined).expect("TODO: panic message");
                window.set_cursor_visible(false);
            }
            MouseButton::Right => {}
            MouseButton::Middle => {}
            MouseButton::Back => {}
            MouseButton::Forward => {}
            MouseButton::Other(_) => {}
        }
    }

    pub fn process_mouse_motion(&mut self, delta: (f32, f32)) {
        println!("{:?}", delta);
    }

    pub fn update(&mut self) {}
}