use crate::gpu::camera::controller::CameraController;
use crate::gpu::camera::position::CameraPosition;
use crate::gpu::camera::projection::Projection;
use crate::gpu::camera::{Camera, CameraUniform};
use crate::gpu::error::{GpuError, GpuResult};
use crate::gpu::initializer::create_state;
use crate::gpu::vertex::Vertex;
use log::{error, info};
use nalgebra::{
    Matrix4, Orthographic3, Perspective3, Point3, Translation3, UnitQuaternion, Vector3,
};
use std::iter;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::Surface;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, Event, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub struct GpuProcessor {
    state: State,
}
enum State {
    NotInitialized,
    Failed(GpuError),
    Initialized(GpuHandler),
}
pub struct GpuHandler {
    window: Arc<Window>,
    instance: wgpu::Instance,
    surface: Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
    camera: Camera,
}

impl GpuHandler {
    pub fn new(
        window: Arc<Window>,
        instance: wgpu::Instance,
        surface: Surface<'static>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        config: wgpu::SurfaceConfiguration,
        size: winit::dpi::PhysicalSize<u32>,
        pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
        num_vertices: u32,
        num_indices: u32,
        camera: Camera,
    ) -> Self {
        Self {
            window,
            instance,
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            vertex_buffer,
            num_vertices,
            num_indices,
            camera,
        }
    }
}

impl GpuHandler {
    fn render(&mut self) -> GpuResult<()> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.camera.camera_bind_group(), &[]);
            render_pass.draw(0..self.num_vertices, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            info!("Resizing to {:?}", new_size);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.resize(new_size.width, new_size.height);
            self.update()
        }
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => self.camera.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.camera
                    .set_mouse_pressed(*state == ElementState::Pressed);
                true
            }
            _ => false,
        }
    }
    fn update(&mut self) {
        self.camera.update_camera();
        self.queue.write_buffer(
            &self.camera.camera_buffer(),
            0,
            bytemuck::cast_slice(&[*self.camera.uniform()]),
        );
    }
    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }
}

impl GpuProcessor {
    pub fn state(&mut self) -> GpuResult<&mut GpuHandler> {
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
}

impl Default for GpuProcessor {
    fn default() -> Self {
        GpuProcessor {
            state: State::NotInitialized,
        }
    }
}

impl ApplicationHandler for GpuProcessor {
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
