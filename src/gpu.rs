use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod gui;
mod processor;
mod vertex;
mod light;

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
pub struct GpuOptions {}

impl Default for GpuOptions {
    fn default() -> Self {
        Self {}
    }
}
