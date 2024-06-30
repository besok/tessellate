use crate::gpu::processor::GpuProcessor;
use std::error::Error;
use std::fmt::Display;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

mod camera;
mod processor;
mod vertex;
mod error;

pub async fn run() -> Result<(), EventLoopError> {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut GpuProcessor::default())
}
