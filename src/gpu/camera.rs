use nalgebra::{Matrix4, Point3, Vector3};

struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
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