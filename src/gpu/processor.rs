use winit::application::ApplicationHandler;
use winit::window::Window;

use crate::gpu::GpuException;

mod init;
mod logic;

#[derive(Default)]
pub struct GpuProcessor<'a> {
    state: State<'a>,
}

enum State<'a> {
    NonInitialized,
    Initialized {
        window: &'a Window
    },
    Failed(GpuException),
}

impl State {
    pub fn init(window: &Window) -> Self {
        State::Initialized { window }
    }
    pub(crate) fn is_failed(&self) -> Option<&GpuException> {
        match self {
            State::NonInitialized | State::Initialized { .. } => { None }
            State::Failed(e) => { Some(e) }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::NonInitialized
    }
}




