use std::error::Error;
use std::fmt::{Display, Formatter};

mod state;
mod vertex;
mod runner;
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


pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }
    runner::run().await;
}
