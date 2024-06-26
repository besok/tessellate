use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    pub(crate) eye: Point3<f32>,
    pub(crate) target: Point3<f32>,
    pub(crate) up: Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {

        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);

        let proj =
            nalgebra::Perspective3::new(self.aspect,
                                        self.fovy.to_radians(),
                                        self.znear,
                                        self.zfar).into_inner();


        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);