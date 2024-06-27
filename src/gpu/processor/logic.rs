use log::info;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};
use crate::gpu::GpuException;
use crate::gpu::processor::{GpuProcessor, State};
use crate::gpu::processor::init::init_state;

impl ApplicationHandler for GpuProcessor {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match init_state(event_loop) {
            Ok(state) => {
                self.state = state;
            }
            Err(e) => {
                self.state = State::Failed(e);
            }
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(e) = self.state.is_failed() {
            info!("GPU processor failed: {}", e);
            event_loop.exit();
        } else {
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
                    // // This tells winit that we want another frame after this one
                    // state.window().request_redraw();
                    //
                    // if !surface_configured {
                    //     return;
                    // }
                    //
                    // state.update();
                    // match state.render() {
                    //     Ok(_) => {}
                    //     // Reconfigure the surface if it's lost or outdated
                    //     Err(
                    //         wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                    //     ) => state.resize(state.size),
                    //     // The system is out of memory, we should probably quit
                    //     Err(wgpu::SurfaceError::OutOfMemory) => {
                    //         log::error!("OutOfMemory");
                    //         control_flow.exit();
                    //     }
                    //
                    //     // This happens when the a frame takes too long to present
                    //     Err(wgpu::SurfaceError::Timeout) => {
                    //         log::warn!("Surface timeout")
                    //     }
                    // }
                }

                _ => (),
            }
        }
    }
}
