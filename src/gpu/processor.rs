use crate::gpu::camera::position::CameraPosition;
use crate::gpu::camera::Camera;
use crate::gpu::error::{GpuError, GpuResult};
use crate::mesh::Mesh;
use log::{error, info};
use std::collections::HashMap;

use crate::gpu::gui::GuiRenderer;
use crate::gpu::light::Light;
use crate::gpu::material::Material;
use crate::gpu::vertex::GpuVertex;
use crate::gpu::options::GpuOptions;
use crate::mesh::attributes::MeshType;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::{Buffer, RenderPipeline, Surface};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

mod init;
mod render;

pub struct GpuProcessor {
    state: State,
}

struct GpuMesh {
    vertex_buffer: Buffer,
    mesh: Mesh,
    vertices: Vec<GpuVertex>,
    material: Material,
    affected_by_light: wgpu::BindGroup,
}

impl GpuMesh {
    pub fn new(
        vertex_buffer: Buffer,
        vertices: Vec<GpuVertex>,
        mesh: Mesh,
        material: Material,
        affected_by_light: wgpu::BindGroup,
    ) -> Self {
        GpuMesh {
            vertex_buffer,
            mesh,
            vertices,
            material,
            affected_by_light
        }
    }
}

impl GpuProcessor {
    pub fn new(meshes: Vec<Mesh>, camera: CameraPosition, options: GpuOptions) -> Self {
        GpuProcessor {
            state: State::NotInitialized(meshes, camera, options),
        }
    }
}

enum State {
    NotInitialized(Vec<Mesh>, CameraPosition, GpuOptions),
    Failed(GpuError),
    Initialized(GpuHandler),
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Topology {
    LineList,
    TriangleList,
}
pub struct GpuHandler {
    window: Arc<Window>,
    instance: wgpu::Instance,
    surface: Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: dpi::PhysicalSize<u32>,
    pipelines: HashMap<Topology, RenderPipeline>,
    meshes: Vec<GpuMesh>,
    camera: Camera,
    light: Light,
    gui: GuiRenderer,
    gpu_options: GpuOptions,
}

impl GpuHandler {
    pub fn new(
        window: Arc<Window>,
        instance: wgpu::Instance,
        surface: Surface<'static>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
        size: dpi::PhysicalSize<u32>,
        pipelines: HashMap<Topology, RenderPipeline>,
        meshes: Vec<GpuMesh>,
        camera: Camera,
        gui: GuiRenderer,
        light: Light,
        gpu_options: GpuOptions,
    ) -> Self {
        Self {
            window,
            instance,
            surface,
            device,
            queue,
            config,
            size,
            pipelines,
            meshes,
            camera,
            gui,
            light,
            gpu_options
        }
    }
}

impl GpuProcessor {
    pub fn state(&mut self) -> GpuResult<&mut GpuHandler> {
        match &mut self.state {
            State::NotInitialized(..) => {
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
}

impl Default for GpuProcessor {
    fn default() -> Self {
        GpuProcessor {
            state: State::NotInitialized(vec![], CameraPosition::default(), GpuOptions::default()),
        }
    }
}

impl ApplicationHandler for GpuProcessor {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match &self.state {
            State::NotInitialized(meshes, camera, opts) => {
                match GpuProcessor::try_init(event_loop, meshes, camera.clone(), opts.clone()) {
                    Ok(s) => {
                        self.state = State::Initialized(s);
                    }
                    Err(e) => {
                        self.state = State::Failed(e);
                    }
                }
            }

            _ => {
                info!("GPU processor already initialized ",);
                return;
            }
        }
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match self.state() {
            Ok(s) => {
                s.gui.handle_input(&mut s.window, &event);
                if _id == s.window().id() && !s.input(&event) {
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
                            s.resize(physical_size);
                        }
                        WindowEvent::RedrawRequested if _id == s.window().id() => {
                            s.window.request_redraw();
                            s.update();
                            match s.render() {
                                Ok(_) => {}
                                Err(GpuError::General(e)) => {
                                    error!("Render failed: {e}");
                                    event_loop.exit();
                                }
                                Err(GpuError::WgpuSurfaceError(e)) => match e {
                                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                                        s.resize(s.size);
                                    }
                                    wgpu::SurfaceError::OutOfMemory => {
                                        error!("OutOfMemory");
                                        event_loop.exit();
                                    }
                                    wgpu::SurfaceError::Timeout => {
                                        error!("Surface timeout");
                                    }
                                },
                                Err(GpuError::EventLoopError(e)) => {
                                    error!("EventLoop failed: {e}");
                                    event_loop.exit();
                                }
                                Err(GpuError::MeshError(e)) => {
                                    error!("Mesh error: {e}");
                                    event_loop.exit();
                                }
                            }
                        }

                        _ => (),
                    }
                }
            }
            Err(e) => {
                info!("GPU processor failed: {}", e);
                event_loop.exit();
            }
        }
    }
}
