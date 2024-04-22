use std::f32::consts::FRAC_PI_2;
use glam::{
    Mat4,
    Vec3,
    Vec4,
};

pub struct Camera {
    position: Vec3,
    rotation: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        return Self {
            position: Vec3::ZERO,
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
}

pub struct CameraController {
    movement_speed: f32,
    rotation_speed: f32,
}

impl CameraController {}