use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
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
}

impl GpuOptions {
    pub fn new(camera_speed: f32, camera_sensitivity: f32) -> Self {
        Self {
            camera_speed,
            camera_sensitivity,
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
}

impl Default for GpuOptions {
    fn default() -> Self {
        Self {
            camera_speed: 0.1,
            camera_sensitivity: 0.001,
        }
    }
}
