use glam::{Mat4, Vec3};

pub struct CameraPosition {
    position: Vec3,
    yaw: f32,
    pitch: f32,
}

impl CameraPosition {
    pub fn new(position: Vec3, yaw: f32, pitch: f32) -> Self {
        Self {
            position,
            yaw,
            pitch,
        }
    }

    pub fn update_position(&mut self, shift: Vec3) {
        self.position += shift;
    }
    pub fn update_yaw(&mut self, shift: f32) {
        self.yaw = self.yaw + shift;
    }
    pub fn update_pitch(&mut self, shift: f32) {
        self.pitch = self.pitch + shift;
    }
    pub fn set_pitch(&mut self, shift: f32) {
        self.pitch = shift;
    }

    pub fn shift_y(&mut self, shift: f32) {
        self.position.y += shift;
    }
    pub fn yaw(&self) -> f32 {
        self.yaw
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn calc_matrix(&self) -> Mat4 {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let center = self.position
            + Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize();

        Mat4::look_at_rh(self.position, center, Vec3::Y)
    }
}
