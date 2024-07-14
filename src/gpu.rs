use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
use env_logger::Builder;
use log::{info, LevelFilter};
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

mod camera;
mod error;
mod processor;
mod vertex;


async fn run(meshes: Vec<Mesh>) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut GpuProcessor::new(meshes))
}

pub fn visualize(meshes: Vec<Mesh>) -> Result<(), EventLoopError> {
    pollster::block_on(run(meshes))
}
