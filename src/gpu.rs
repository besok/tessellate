use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
use options::GpuOptions;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod gui;
mod light;
pub mod material;
pub mod options;
mod processor;
mod vertex;

async fn run(meshes: Vec<Mesh>, options: GpuOptions) -> Result<(), GpuError> {
    let camera_pos = CameraPosition::new(options.camera_opts().position(), 0.0, 0.0);
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    Ok(event_loop.run_app(&mut GpuProcessor::new(meshes, camera_pos, options))?)
}

pub fn visualize(meshes: Vec<Mesh>, options: GpuOptions) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, options))
}
