use env_logger::Builder;
use crate::gpu::processor::GpuProcessor;
use log::{info, LevelFilter};
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

mod camera;
mod processor;
mod vertex;
mod error;
mod instance;

#[cfg(not(target_arch = "wasm32"))]
fn init_logger() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    info!("Logger initialized");
}
#[cfg(target_arch = "wasm32")]
fn init_logger() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    info!("Logger initialized");
}
pub async fn run() -> Result<(), EventLoopError> {
    init_logger();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut GpuProcessor::default())
}
