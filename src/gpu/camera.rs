pub mod controller;
pub mod position;
pub mod projection;
mod utils;

use crate::gpu::camera::controller::CameraController;
use crate::gpu::camera::position::CameraPosition;
use crate::gpu::camera::projection::Projection;
use crate::gpu::camera::utils::OPENGL_TO_WGPU_MATRIX;
use nalgebra::{Matrix4, Point3, Vector3};
use winit::event::{ElementState, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct Camera {
    camera: CameraPosition,
    projection: Projection,
    uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: CameraController,
    mouse_pressed: bool,
}

impl Camera {
    pub fn new(
        camera: CameraPosition,
        uniform: CameraUniform,
        camera_buffer: wgpu::Buffer,
        camera_bind_group: wgpu::BindGroup,
        projection: Projection,
        camera_controller: CameraController,
    ) -> Self {
        Self {
            camera,
            uniform,
            camera_buffer,
            camera_bind_group,
            camera_controller,
            projection,
            mouse_pressed: false,
        }
    }

    pub fn is_mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }

    pub fn camera_bind_group(&self) -> &wgpu::BindGroup {
        &self.camera_bind_group
    }
    pub fn camera_controller(&mut self) -> &mut CameraController {
        &mut self.camera_controller
    }
    pub fn camera(&mut self) -> &mut CameraPosition {
        &mut self.camera
    }
    pub fn uniform(&self) -> &CameraUniform {
        &self.uniform
    }
    pub fn camera_buffer(&self) -> &wgpu::Buffer {
        &self.camera_buffer
    }
    pub fn update_camera(&mut self, dt: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera,dt);
        self.uniform.update_view_proj(&self.camera,&self.projection);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.projection.resize(width, height);
    }
    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
        self.camera_controller.process_keyboard(key, state)
    }
    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.camera_controller.process_scroll(delta);
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.camera_controller.process_mouse(mouse_dx, mouse_dy);
    }

    pub fn set_mouse_pressed(&mut self, pressed: bool) {
        self.mouse_pressed = pressed;
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

    pub(crate) fn update_view_proj(&mut self, camera: &CameraPosition, projection: &Projection) {
        // self.view_position = camera.to_homogeneous().into();
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).into();
    }
}
