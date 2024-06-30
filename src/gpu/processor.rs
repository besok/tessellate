use crate::gpu::error::{GpuError, GpuResult};
use log::{error, info};
use std::sync::Arc;
use wgpu::{CreateSurfaceError, Surface};
use winit::application::ApplicationHandler;
use winit::error::OsError;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub struct GpuProcessor {
    state: State ,
}
enum State  {
    NotInitialized,
    Failed(GpuError),
    Initialized(Delegate ),
}
struct Delegate  {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}
impl GpuProcessor  {
    pub fn state(&mut self) -> GpuResult<& mut Delegate> {
        match &mut self.state {
            State::NotInitialized => {
                error!("GPU processor not initialized");
                Err(GpuError::new("GPU processor not initialized"))
            }
            State::Failed(e) => {
                error!("GPU processor failed: {}", e);
                Err(e.clone())
            }
            State::Initialized(ref mut s) => Ok(s),
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}

impl Default for GpuProcessor {
    fn default() -> Self {
        GpuProcessor {
            state: State::NotInitialized,
        }
    }
}

fn create_state(event_loop: &ActiveEventLoop) -> GpuResult<Delegate> {
    let window = Arc::new(event_loop.create_window(Window::default_attributes())?);
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        #[cfg(not(target_arch = "wasm32"))]
        backends: wgpu::Backends::PRIMARY,
        #[cfg(target_arch = "wasm32")]
        backends: wgpu::Backends::GL,
        ..Default::default()
    });
    let s = instance.create_surface(window.clone())?;
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&s),
        force_fallback_adapter: false,
    }))
    .ok_or(GpuError::new("Failed to request adapter"))?;

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web, we'll have to disable some.
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None,
        },
        None, // Trace path
    ))?;

    let surface_caps = s.get_capabilities(&adapter);
    // Shader code in this tutorial assumes an sRGB surface texture. Using a different
    // one will result in all the colors coming out darker. If you want to support non
    // sRGB surfaces, you'll need to account for that when drawing to the frame.
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    Ok(Delegate {
        window,
        surface: s,
        device,
        queue,
        config,
        size,
    })
}

impl ApplicationHandler for GpuProcessor{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match create_state(event_loop) {
            Ok(s) => {
                self.state = State::Initialized(s);
            }
            Err(e) => {
                self.state = State::Failed(e);
            }
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match self.state() {
            Ok(s) => {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: PhysicalKey::Code(KeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        info!("The close button was pressed; stopping");
                        event_loop.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        if physical_size.width > 0 && physical_size.height > 0 {
                            info!("Trying to resize the window to {:?}", physical_size);

                            let mut config = &mut s.config;
                            config.width = physical_size.width;
                            config.height = physical_size.height;
                            s.size = physical_size;
                            // self.surface.configure(&self.device, &self.config);
                        }
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
            Err(e) => {
                info!("GPU processor failed: {}", e);
                event_loop.exit();
            }
        }
    }
}
