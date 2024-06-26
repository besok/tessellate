use log::info;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use crate::gpu::GpuException;
#[derive(Default)]
pub struct GpuProcessor {
    window: Option<Window>,
}

impl ApplicationHandler for GpuProcessor {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window =
            event_loop
                .create_window(Window::default_attributes())
                .expect("Failed to create window");
        self.window = Some(window);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape), ..
                }, ..
            } => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                // surface_configured = true;
                // state.resize(*physical_size);
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }

            _ => (),
        }
    }
}
