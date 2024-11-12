use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::material::{Color, RgbaColor};
use crate::mesh::Mesh;
use glam::Vec3;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod gui;
mod light;
mod processor;
mod vertex;

async fn run(
    meshes: Vec<Mesh>,
    camera: CameraPosition,
    options: GpuOptions,
) -> Result<(), GpuError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    Ok(event_loop.run_app(&mut GpuProcessor::new(meshes, camera, options))?)
}

pub fn visualize(meshes: Vec<Mesh>, camera: CameraPosition) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, camera, GpuOptions::default()))
}

pub fn visualize_with(
    meshes: Vec<Mesh>,
    camera: CameraPosition,
    options: GpuOptions,
) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, camera, options))
}

#[derive(Debug, Clone)]
pub struct GpuOptions {
    camera_speed: f32,
    camera_sensitivity: f32,
    light: Light,
}
#[derive(Debug, Clone)]
pub struct Light {
    color: RgbaColor,
    position: Vec3,
}

impl Light {
    pub fn new(color: RgbaColor, position: Vec3) -> Self {
        Self { color, position }
    }
    pub fn with_color(&mut self, color: RgbaColor) -> &mut Self {
        self.color = color;
        self
    }
    pub fn with_position(&mut self, position: Vec3) -> &mut Self {
        self.position = position;
        self
    }
}

impl GpuOptions {
    pub fn new(camera_speed: f32, camera_sensitivity: f32, light: Light) -> Self {
        Self {
            camera_speed,
            camera_sensitivity,
            light,
        }
    }
    pub fn with_camera_speed(&mut self, camera_speed: f32) -> &mut Self {
        self.camera_speed = camera_speed;
        self
    }

    pub fn with_camera_sensitivity(&mut self, camera_sensitivity: f32) -> &mut Self {
        self.camera_sensitivity = camera_sensitivity;
        self
    }

    pub fn with_light(&mut self, light: Light) -> &mut Self {
        self.light = light;
        self
    }
}

impl Default for GpuOptions {
    fn default() -> Self {
        Self {
            camera_speed: 0.1,
            camera_sensitivity: 0.001,
            light: Light::new(RgbaColor::WHITE, Vec3::new(5.0, 5.0, 5.0)),
        }
    }
}
