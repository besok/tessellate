use crate::gpu::camera::{Camera, CameraController, CameraHandler, CameraUniform};
use crate::gpu::error::{GpuError, GpuResult};
use crate::gpu::vertex::{Vertex, INDICES, VERTICES};
use log::{error, info};
use std::iter;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use wgpu::Surface;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
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
struct GpuHandler {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
    index_buffer: wgpu::Buffer,
    camera_handler: CameraHandler,
}

impl GpuHandler {
    fn render(&mut self) -> GpuResult<()> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

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
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.set_bind_group(0, &self.camera_handler.camera_bind_group(), &[]);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
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
        }
    }
    fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_handler.camera_controller().process_events(event)
    }
    fn update(&mut self) {
        self.camera_handler.update_camera();
        self.queue.write_buffer(&self.camera_handler.camera_buffer(), 0, bytemuck::cast_slice(&[
            *self.camera_handler.uniform()
        ]));
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
            Ok(s)  => {
                if !s.input(&event) {
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
                        WindowEvent::RedrawRequested => {
                            // This tells winit that we want another frame after this one
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
fn create_state(event_loop: &ActiveEventLoop) -> GpuResult<GpuHandler> {
    let window = Arc::new(event_loop.create_window(Window::default_attributes())?);
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        #[cfg(not(target_arch = "wasm32"))]
        backends: wgpu::Backends::PRIMARY,
        #[cfg(target_arch = "wasm32")]
        backends: wgpu::Backends::GL,
        ..Default::default()
    });
    let surface = instance.create_surface(window.clone())?;
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
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

    let surface_caps = surface.get_capabilities(&adapter);
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

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });


    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let num_vertices = VERTICES.len() as u32;
    let num_indices = INDICES.len() as u32;
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });
    let camera = Camera::center();
    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera);

    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }
        ],
        label: Some("camera_bind_group"),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            &camera_bind_group_layout,
        ],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // or Features::POLYGON_MODE_POINT
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    });
    let camera_controller = CameraController::new(0.2);
    let camera_handler = CameraHandler::new(camera, camera_uniform, camera_buffer, camera_bind_group,camera_controller);

    Ok(GpuHandler {
        window,
        surface,
        device,
        queue,
        config,
        render_pipeline,
        size,
        vertex_buffer,
        num_vertices,
        index_buffer,
        num_indices,
        camera_handler,
    })
}
