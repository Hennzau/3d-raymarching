use std::f32::consts::{FRAC_PI_2, PI};

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
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct CameraController {
    speed: f32,

    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,

    rotate_right: bool,
    rotate_left: bool,
    rotate_up: bool,
    rotate_down: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        return Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,

            rotate_right: false,
            rotate_left: false,
            rotate_up: false,
            rotate_down: false,
        };
    }

    pub fn process(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                logical_key,
                physical_key,
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
                match physical_key {
                    PhysicalKey::Code(KeyCode::Space) => {
                        self.is_up_pressed = (state == ElementState::Pressed);
                    }
                    PhysicalKey::Code(KeyCode::ShiftLeft) => {
                        self.is_down_pressed = (state == ElementState::Pressed);
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.rotate_left = (state == ElementState::Pressed);
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.rotate_right = (state == ElementState::Pressed);
                    }
                    PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.rotate_up = (state == ElementState::Pressed);
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.rotate_down = (state == ElementState::Pressed);
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
        if self.is_up_pressed {
            camera.move_up(self.speed);
        }
        if self.is_down_pressed {
            camera.move_down(self.speed);
        }


        if self.rotate_up {
            camera.rotation += Vec3::new(0.1f32, 0f32, 0f32);
        }
        if self.rotate_down {
            camera.rotation += Vec3::new(-0.1f32, 0f32, 0f32);
        }
        if self.rotate_left {
            camera.rotation += Vec3::new(0f32, 0f32, 0.1f32);
        }
        if self.rotate_right {
            camera.rotation += Vec3::new(0f32, 0f32, -0.1f32);
        }

        camera.update_projection_view_matrix();
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
            position: Vec3::new(0f32, -3f32, 0f32),
            rotation: Vec3::new(FRAC_PI_2, 0f32, 0f32),

            projection_view_matrix: Mat4::IDENTITY,
            aspect_ratio,
        };
    }

    pub fn process_resize(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.update_projection_view_matrix();
    }

    fn update_projection_view_matrix(&mut self) {
        let projection = Mat4::perspective_infinite_rh(70f32 * PI / 180f32, self.aspect_ratio, 0.01);
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let position = Mat4::from_translation(-self.position);

        self.projection_view_matrix = projection * rotation * position;
    }

    pub fn move_forward(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let forward = rotation.transpose() * Vec4::new(0f32, 0f32, -speed, 0f32);
        let mut forward = forward.truncate();

        forward.z = 0f32;

        self.position += forward;
    }

    pub fn move_backward(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let backward = rotation.transpose() * Vec4::new(0f32, 0f32, speed, 0f32);
        let mut backward = backward.truncate();

        backward.z = 0f32;

        self.position += backward;
    }

    pub fn move_left(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let forward = rotation.transpose() * Vec4::new(0f32, 0f32, -speed, 0f32);
        let mut forward = forward.truncate();

        forward.z = 0f32;

        let left = Vec3::Z.cross(forward);

        self.position += left;
    }

    pub fn move_right(&mut self, speed: f32) {
        let rotation = Mat4::from_rotation_x(-self.rotation.x) * Mat4::from_rotation_y(-self.rotation.y) * Mat4::from_rotation_z(-self.rotation.z);
        let forward = rotation.transpose() * Vec4::new(0f32, 0f32, -speed, 0f32);
        let mut forward = forward.truncate();

        forward.z = 0f32;

        let right = forward.cross(Vec3::Z);

        self.position += right;
    }

    pub fn move_up(&mut self, speed: f32) {
        self.position += Vec3::new(0f32, 0f32, speed);
    }

    pub fn move_down(&mut self, speed: f32) {
        self.position += Vec3::new(0f32, 0f32, -speed);
    }
}