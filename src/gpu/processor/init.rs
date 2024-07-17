use crate::gpu::camera::position::CameraPosition;
use crate::gpu::camera::Camera;
use crate::gpu::error::{GpuError, GpuResult};
use crate::gpu::processor::{GpuHandler, GpuMesh, GpuProcessor};
use crate::gpu::vertex::{Vertex};
use crate::mesh::Mesh;
use std::sync::Arc;
use wgpu::Features;
use wgpu::util::DeviceExt;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;


pub struct Settings{
    // if true, required_features: wgpu::Features::POLYGON_MODE_LINE and polygon_mode: wgpu::PolygonMode::Line
    pub only_lines:bool
}

impl GpuProcessor {
    pub fn try_init(
        event_loop: &ActiveEventLoop,
        meshes: &Vec<Mesh>,
        camera_pos: CameraPosition,
    ) -> GpuResult<GpuHandler> {
        let attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(attributes)?);
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
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
                required_features: Features::empty(), // wgpu::Features::POLYGON_MODE_LINE,
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ))?;

        let surface_caps = surface.get_capabilities(&adapter);
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
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsl/shader.wgsl").into()),
        });

        let mut gpu_meshes = Vec::new();

        for mesh in meshes.into_iter() {
            let vertices:Vec<Vertex> =mesh.try_into()?;
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            gpu_meshes.push(GpuMesh::new(vertex_buffer, mesh.clone()));
        }
        let camera = Camera::init(&config, &device, camera_pos);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera.camera_bind_layout()],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                polygon_mode: wgpu::PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(GpuHandler::new(
            window,
            instance,
            surface,
            device,
            queue,
            config,
            size,
            pipeline,
            gpu_meshes,
            camera,
        ))
    }
}
