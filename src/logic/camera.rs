use std::f32::consts::{FRAC_PI_2, PI};

use glam::{
    Mat4,
    Vec3,
    Vec4
};

use winit::{
    event::{
        ElementState,
        KeyEvent,
    },
    keyboard::{
        KeyCode,
        PhysicalKey,
    },
};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        return Self {
            position: Vec3::new(0f32, -3f32, 0f32),
            rotation: Vec3::new(FRAC_PI_2, 0f32, 0f32),
        };
    }

    pub fn move_forward(&mut self, delta: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let forward = rotation.transpose() * Vec4::new(0f32, 0f32, -delta, 0f32);
        let mut forward = forward.truncate();

        forward.z = 0f32;

        self.position += forward;
    }

    pub fn move_backward(&mut self, delta: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let backward = rotation.transpose() * Vec4::new(0f32, 0f32, delta, 0f32);
        let mut backward = backward.truncate();

        backward.z = 0f32;

        self.position += backward;
    }

    pub fn move_left(&mut self, delta: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let forward = rotation.transpose() * Vec4::new(0f32, 0f32, -delta, 0f32);
        let mut forward = forward.truncate();

        forward.z = 0f32;

        let left = Vec3::Z.cross(forward);

        self.position += left;
    }

    pub fn move_right(&mut self, delta: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let forward = rotation.transpose() * Vec4::new(0f32, 0f32, -delta, 0f32);
        let mut forward = forward.truncate();

        forward.z = 0f32;

        let right = forward.cross(Vec3::Z);

        self.position += right;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.position += Vec3::new(0f32, 0f32, delta);
    }

    pub fn move_down(&mut self, delta: f32) {
        self.position += Vec3::new(0f32, 0f32, -delta);
    }

    pub fn rotate_horizontally(&mut self, angle: f32) {
        self.rotation += Vec3::new(0f32, 0f32, angle);
    }

    pub fn rotate_vertically(&mut self, angle: f32) {
        self.rotation += Vec3::new(angle, 0f32, 0f32);
    }

    pub fn get_inverted_projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        let projection = Mat4::perspective_infinite_rh(70f32 * PI / 180f32, aspect_ratio, 0.01);

        return projection.inverse();
    }

    pub fn get_inverted_view_matrix(&self) -> Mat4 {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let position = Mat4::from_translation(-self.position);

        return (rotation * position).inverse();
    }

    pub fn build_projection_view_matrix(&self, aspect_ratio: f32) -> Mat4 {
        let projection = Mat4::perspective_infinite_rh(70f32 * PI / 180f32, aspect_ratio, 0.01);
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let position = Mat4::from_translation(-self.position);

        return projection * rotation * position;
    }
}

pub struct CameraController {
    movement_speed: f32,
    rotation_speed: f32,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,

    horizontal_delta: f32,
    vertical_delta: f32,
}

impl CameraController {
    pub fn new() -> Self {
        return Self {
            movement_speed: 50f32,
            rotation_speed: 1f32,

            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,

            horizontal_delta: 0f32,
            vertical_delta: 0f32,
        };
    }

    pub fn process_keyboard(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                physical_key,
                state,
                ..
            } => {
                match physical_key {
                    PhysicalKey::Code(KeyCode::KeyW) => {
                        self.is_forward_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyS) => {
                        self.is_backward_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyA) => {
                        self.is_left_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyD) => {
                        self.is_right_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::Space) => {
                        self.is_up_pressed = state == ElementState::Pressed;
                    }
                    PhysicalKey::Code(KeyCode::ShiftLeft) => {
                        self.is_down_pressed = state == ElementState::Pressed;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32, camera: &mut Camera) {
        if self.is_forward_pressed {
            camera.move_forward(self.movement_speed * delta_time);
        }
        if self.is_backward_pressed {
            camera.move_backward(self.movement_speed * delta_time);
        }
        if self.is_left_pressed {
            camera.move_left(self.movement_speed * delta_time);
        }
        if self.is_right_pressed {
            camera.move_right(self.movement_speed * delta_time);
        }
        if self.is_up_pressed {
            camera.move_up(self.movement_speed * delta_time);
        }
        if self.is_down_pressed {
            camera.move_down(self.movement_speed * delta_time);
        }

        camera.rotate_horizontally(self.horizontal_delta * delta_time * self.rotation_speed);
        camera.rotate_vertically(self.vertical_delta * delta_time * self.rotation_speed);

        self.horizontal_delta = 0f32;
        self.vertical_delta = 0f32;
    }

    pub fn process_mouse_motion(&mut self, delta: (f32, f32)) {
        self.horizontal_delta = -delta.0 * self.rotation_speed;
        self.vertical_delta = -delta.1 * self.rotation_speed;
    }
}