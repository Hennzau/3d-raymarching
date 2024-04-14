use std::f32::consts::PI;

use glam::{
    Mat4,
    Vec3,
    Vec4,
};

use winit::{
    event::{
        ElementState,
        KeyEvent,
    },
    keyboard::Key,
};

pub struct CameraController {
    speed: f32,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        return Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        };
    }

    pub fn process(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                logical_key,
                state,
                ..
            } => {
                match logical_key.as_ref() {
                    Key::Character("z") => {
                        self.is_forward_pressed = (state == ElementState::Pressed);
                    }
                    Key::Character("s") => {
                        self.is_backward_pressed = (state == ElementState::Pressed);
                    }
                    Key::Character("q") => {
                        self.is_left_pressed = (state == ElementState::Pressed);
                    }
                    Key::Character("d") => {
                        self.is_right_pressed = (state == ElementState::Pressed);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(&self, camera: &mut Camera) {
        if self.is_forward_pressed {
            camera.move_forward(self.speed);
        }
        if self.is_backward_pressed {
            camera.move_backward(self.speed);
        }
        if self.is_left_pressed {
            camera.move_left(self.speed);
        }
        if self.is_right_pressed {
            camera.move_right(self.speed);
        }
    }
}

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,

    pub projection_view_matrix: Mat4,
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Camera {
        return Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,

            projection_view_matrix: Mat4::IDENTITY,
            aspect_ratio,
        };
    }

    pub fn process_resize(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.update_projection_view_matrix();
    }

    fn update_projection_view_matrix(&mut self) {
        let projection = glam::Mat4::perspective_rh(70f32 * PI / 180f32, self.aspect_ratio, 0.01, 100.0);
        let rotation = Mat4::from_rotation_x(self.rotation.x) * Mat4::from_rotation_y(self.rotation.y) * Mat4::from_rotation_z(self.rotation.z);
        let position = Mat4::from_translation(self.position);

        self.projection_view_matrix = projection * rotation * position;
    }

    pub fn move_forward(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(self.rotation.x) * Mat4::from_rotation_y(self.rotation.y) * Mat4::from_rotation_z(self.rotation.z);
        let forward = rotation * Vec4::new(speed, 0f32, 0f32, 0f32);
        let forward = forward.truncate();

        self.position += forward;

        self.update_projection_view_matrix();
    }

    pub fn move_backward(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(self.rotation.x) * Mat4::from_rotation_y(self.rotation.y) * Mat4::from_rotation_z(self.rotation.z);
        let backward = rotation * Vec4::new(-speed, 0f32, 0f32, 0f32);
        let backward = backward.truncate();

        self.position += backward;

        self.update_projection_view_matrix();
    }

    pub fn move_left(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(self.rotation.x) * Mat4::from_rotation_y(self.rotation.y) * Mat4::from_rotation_z(self.rotation.z);
        let left = rotation * Vec4::new(0f32, speed, 0f32, 0f32);
        let left = left.truncate();

        self.position += left;

        self.update_projection_view_matrix();
    }

    pub fn move_right(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(self.rotation.x) * Mat4::from_rotation_y(self.rotation.y) * Mat4::from_rotation_z(self.rotation.z);
        let right = rotation * Vec4::new(0f32, -speed, 0f32, 0f32);
        let right = right.truncate();

        self.position += right;

        self.update_projection_view_matrix();
    }
}