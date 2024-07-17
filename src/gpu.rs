use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
use env_logger::Builder;
use glam::Vec3;
use log::{info, LevelFilter};
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod processor;
mod vertex;

async fn run(meshes: Vec<Mesh>, camera: CameraPosition) -> Result<(), GpuError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    Ok(event_loop.run_app(&mut GpuProcessor::new(meshes, camera))?)
}

pub fn visualize(meshes: Vec<Mesh>, camera: CameraPosition) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, camera))
}
