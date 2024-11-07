use crate::gpu::camera::position::CameraPosition;
use crate::gpu::camera::Camera;
use crate::gpu::error::{GpuError, GpuResult};
use crate::gpu::gui::GuiRenderer;
use crate::gpu::processor::{GpuHandler, GpuMesh, GpuProcessor, Topology};
use crate::gpu::vertex::{GpuInstance, GpuVertex};
use crate::mesh::attributes::MeshType;
use crate::mesh::material::{Color, RgbaColor};
use crate::mesh::parts::bbox::BoundingBox;
use crate::mesh::parts::edge::Edge;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::shape::sphere::Sphere;
use crate::mesh::{Mesh, MeshError, MeshResult};
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::naga::back::glsl::Features;
use egui_wgpu::wgpu::util::DeviceExt;
use ico::IconDir;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::Path;
use std::sync::Arc;
use winit::dpi;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Icon, Window};
use crate::gpu::GpuOptions;

impl GpuProcessor {
    pub fn try_init(
        event_loop: &ActiveEventLoop,
        meshes: &Vec<Mesh>,
        camera_pos: CameraPosition,
        options: GpuOptions,
    ) -> GpuResult<GpuHandler> {
        let attributes = Window::default_attributes()
            .with_title("Tessellate")
            .with_inner_size(dpi::PhysicalSize::new(1600, 1200))
            .with_window_icon(Some(load_icon(Path::new("assets/icon.ico"))?));

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
                required_features: wgpu::Features::empty(), // wgpu::Features::POLYGON_MODE_LINE,
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
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

        let shader_poly = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Poly"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsl/shader_poly.wgsl").into()),
        });
        let shader_vertex = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Vertex"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../wgsl/shader_vertex.wgsl").into()),
        });

        let mut gpu_meshes = Vec::new();

        let aabb = meshes
            .iter()
            .map(|m| m.aabb())
            .reduce(|a, b| (a, b).into())
            .ok_or(MeshError::Custom("No bounding box found".to_string()))?;

        for mesh in meshes.into_iter().chain(create_coord(&aabb).iter()) {
            match mesh.attributes().mesh_type() {
                MeshType::Polygons | MeshType::Lines => {
                    let vertices: Vec<GpuVertex> = mesh.try_into()?;
                    let vertex_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        });
                    gpu_meshes.push(GpuMesh::new(vertex_buffer, vertices, mesh.clone()));
                }
                MeshType::Cloud(size) => {
                    let color = mesh.color();
                    let vertices_sphere: Vec<Mesh> = mesh
                        .vertices()
                        .into_iter()
                        .map(|v| {
                            Sphere::create_uv(v.clone(), size as f32 * 0.01, 8, 8, color.clone())
                        })
                        .map(|m| m.into())
                        .collect();

                    let vertices: Vec<GpuVertex> = vertices_sphere
                        .iter()
                        .map(|m| m.try_into())
                        .collect::<MeshResult<Vec<Vec<GpuVertex>>>>()?
                        .into_iter()
                        .flatten()
                        .collect();

                    let vertex_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                    gpu_meshes.push(GpuMesh::new(vertex_buffer, vertices, mesh.clone()));
                }
            }
        }

        let camera = Camera::init(&config, &device, camera_pos, aabb);

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera.camera_bind_layout()],
                push_constant_ranges: &[],
            });

        let mut pipelines = HashMap::new();
        pipelines.insert(
            Topology::TriangleList,
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline with triangles"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_vertex,
                    entry_point: "vs_main",
                    compilation_options: Default::default(),
                    buffers: &[GpuVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_vertex,
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
                cache: None,
            }),
        );

        pipelines.insert(
            Topology::LineList,
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline with lines"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_vertex,
                    entry_point: "vs_main",
                    compilation_options: Default::default(),
                    buffers: &[GpuVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_vertex,
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
                    topology: wgpu::PrimitiveTopology::LineList,
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
                cache: None,
            }),
        );

        let gui = GuiRenderer::new(&device, config.format, None, 1, window.clone())?;

        Ok(GpuHandler::new(
            window, instance, surface, device, queue, config, size, pipelines, gpu_meshes, camera,
            gui,
        ))
    }
}

fn load_icon(path: &Path) -> GpuResult<Icon> {
    let file = File::open(path)?;
    let icon_dir = IconDir::read(BufReader::new(file))?;
    let icon_image = &icon_dir.entries()[0];
    let icon_rgba = icon_image.decode().expect("Failed to decode icon image");
    Ok(Icon::from_rgba(icon_rgba.rgba_data().to_vec(), icon_image.width(), icon_image.height())?)
}

fn create_coord(aabb: &BoundingBox) -> Vec<Mesh> {
    let m = aabb.min().clone() - 1.0f32;
    let coord = Mesh::lines(
        vec![
            (m.clone(), Vertex::new(m.x + 2.0, m.y, m.z)).into(),
            (m.clone(), Vertex::new(m.x, m.y + 2.0, m.z)).into(),
            (m.clone(), Vertex::new(m.x, m.y, m.z + 2.0)).into(),
        ],
        Color::Line(vec![RgbaColor::RED, RgbaColor::BLUE, RgbaColor::GREEN]),
    );
    vec![coord]
}
