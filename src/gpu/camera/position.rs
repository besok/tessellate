use glam::{Mat4, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct CameraPosition {
    position: Vec3,
    yaw: f32,
    pitch: f32,
}

impl From<Mat4> for CameraPosition {
    fn from(matrix: Mat4) -> Self {
        let position = Vec3::new(matrix.w_axis.x, matrix.w_axis.y, matrix.w_axis.z);
        let forward = Vec3::new(-matrix.z_axis.x, -matrix.z_axis.y, -matrix.z_axis.z).normalize();

        let yaw = -forward.z.atan2(forward.x);
        let pitch = forward.y.asin();

        CameraPosition::new(position, yaw, pitch)
    }
}
impl From<Vec3> for CameraPosition {
    fn from(value: Vec3) -> Self {
        CameraPosition::new(value, 0.0, 0.0)
    }
}
impl Default for CameraPosition {
    fn default() -> Self {
        CameraPosition::new(Vec3::ZERO, 0.0, 0.0)
    }
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
        self.yaw = self.yaw + shift / 300.;
    }
    pub fn update_pitch(&mut self, shift: f32) {
        self.pitch = self.pitch + shift / 300.;
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
