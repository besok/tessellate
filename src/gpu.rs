use std::error::Error;
use std::fmt::{Display, Formatter};
use log::log;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::gpu::processor::GpuProcessor;

mod state;
mod vertex;
mod camera;
mod processor;


#[derive(Debug)]
struct GpuException(String);

impl Display for GpuException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GpuException {}


pub async fn run() -> Result<(), EventLoopError> {
    logger();

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut GpuProcessor::default())
}

fn logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }
}
