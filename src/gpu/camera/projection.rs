use std::f32::consts::FRAC_PI_2;
use nalgebra::{Matrix4, Orthographic3, Perspective3, Point3, Translation3, UnitQuaternion, Vector3};

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let perspective = Perspective3::new(self.aspect, self.fovy, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * perspective.to_homogeneous()

    }
}
#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;



