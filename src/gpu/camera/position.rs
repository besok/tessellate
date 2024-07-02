use nalgebra::{Const, Matrix4, OMatrix, Point3, Vector3};
use std::ops::Add;

pub struct CameraPosition {
    position: Point3<f32>,
    yaw: f32,
    pitch: f32,
}

impl CameraPosition {
    pub fn new(position: Point3<f32>, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
        }
    }

    pub fn update_position(&mut self, shift: OMatrix<f32, Const<3>, Const<1>>) {
        self.position = Point3::from(self.position.coords + shift);
    }
    pub fn update_yaw(&mut self, shift: f32) {
        self.yaw = self.yaw + shift;
    }
    pub fn update_pitch(&mut self, shift: f32) {
        self.pitch = self.pitch + shift;
    }

    pub fn shift_y(&mut self, shift: f32) {
        self.position = Point3::new(self.position.x, self.position.y + shift, self.position.z);
    }
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let forward = Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize();
        let up = Vector3::y();

        let f = forward.normalize();
        let r = up.cross(&f).normalize();
        let u = f.cross(&r).normalize();

        Matrix4::new(
            r.x, u.x, -f.x, 0.0,
            r.y, u.y, -f.y, 0.0,
            r.z, u.z, -f.z, 0.0,
            -r.dot(&self.position.coords), -u.dot(&self.position.coords), f.dot(&self.position.coords), 1.0,
        )
    }
}
