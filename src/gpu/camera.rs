use nalgebra::{Matrix4, Point3, Vector3};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct CameraHandler {
    camera: Camera,
    uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: CameraController
}

impl CameraHandler {
    pub fn new(
        camera: Camera,
        uniform: CameraUniform,
        camera_buffer: wgpu::Buffer,
        camera_bind_group: wgpu::BindGroup,

        camera_controller: CameraController
    ) -> Self {
        Self {
            camera,
            uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller
        }
    }

    pub fn camera_bind_group(&self) -> &wgpu::BindGroup {
        &self.camera_bind_group
    }
    pub fn camera_controller(&mut self) -> &mut CameraController {
        &mut self.camera_controller
    }
    pub fn camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn uniform(&self) -> &CameraUniform {
        &self.uniform
    }
    pub fn camera_buffer(&self) -> &wgpu::Buffer {
        &self.camera_buffer
    }
    pub fn update_camera(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniform.update_view_proj(&self.camera);
    }
}

pub struct Camera {
    eye: Point3<f32>,
    target: Point3<f32>,
    up: Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn center() -> Self {
        Self {
            eye: Point3::new(0.0, 1.0, 2.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }
    pub fn new(
        eye: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }
    fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);

        let proj =
            nalgebra::Perspective3::new(self.aspect, self.fovy.to_radians(), self.znear, self.zfar)
                .into_inner();

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}
impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            view_proj: Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub(crate) fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub(crate) fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    KeyCode::KeyW | KeyCode::ArrowUp => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyA | KeyCode::ArrowLeft => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyS | KeyCode::ArrowDown => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    KeyCode::KeyD | KeyCode::ArrowRight => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    pub(crate) fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(&camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
