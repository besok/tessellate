use glam::{Mat4};
use log::{info};
use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayout, Device, SurfaceConfiguration};
use winit::dpi::PhysicalPosition;
use winit::event::{ MouseScrollDelta};

use crate::gpu::camera::coordinator::{CameraCoordinator};
use crate::gpu::camera::position::CameraPosition;
use crate::gpu::camera::projection::Projection;

pub mod coordinator;
pub mod position;
pub mod projection;



pub struct Camera {
    camera: CameraPosition,
    projection: Projection,
    uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_coord: CameraCoordinator,
    camera_bind_layout: BindGroupLayout,
    mouse_pressed: bool,
}

impl Camera {
    pub fn init(
        config: &SurfaceConfiguration,
        device: &Device,
        camera_pos: CameraPosition
    ) -> Self {
        let projection = Projection::new(config.width, config.height, 45.0, 0.1, 100.0);
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera_pos, &projection);
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
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });
        let coordinator = CameraCoordinator::new(5.0, 0.1, 0.0005);

        Self::new(
            camera_pos,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            projection,
            coordinator,
            camera_bind_group_layout,
        )
    }

    pub fn new(
        camera: CameraPosition,
        uniform: CameraUniform,
        camera_buffer: wgpu::Buffer,
        camera_bind_group: wgpu::BindGroup,
        projection: Projection,
        camera_coord: CameraCoordinator,
        camera_bind_layout: BindGroupLayout,
    ) -> Self {
        Self {
            camera,
            uniform,
            camera_buffer,
            camera_bind_group,
            camera_coord,
            projection,
            camera_bind_layout,
            mouse_pressed: false,
        }
    }

    pub fn is_mouse_pressed(&self) -> bool {
        self.mouse_pressed
    }

    pub fn camera_bind_group(&self) -> &wgpu::BindGroup {
        &self.camera_bind_group
    }
    pub fn camera_bind_layout(&self) -> &wgpu::BindGroupLayout {
        &self.camera_bind_layout
    }

    pub fn camera_coordinator(&mut self) -> &mut CameraCoordinator {
        &mut self.camera_coord
    }
    pub fn camera(&mut self) -> &mut CameraPosition {
        &mut self.camera
    }
    pub fn uniform(&self) -> &CameraUniform {
        &self.uniform
    }
    pub fn camera_buffer(&self) -> &wgpu::Buffer {
        &self.camera_buffer
    }
    pub fn update_camera(&mut self) {
        self.camera_coord.update_camera(&mut self.camera);
        self.uniform
            .update_view_proj(&self.camera, &self.projection);

        info!("Camera position: {:?}", self.camera);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.projection.resize(width, height);
    }
    // pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
    //     self.camera_coord.process_keyboard(key, state)
    // }
    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.camera_coord.process_scroll(delta);
    }

    pub fn process_mouse(&mut self, position:&PhysicalPosition<f64>) -> bool{
        self.camera_coord.process_mouse(position)
    }

    pub fn set_mouse_pressed(&mut self, pressed: bool) -> bool {
        self.mouse_pressed = pressed;
        if !pressed {
            self.camera_coord.clean_mouse_pos();
        }
        true
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}
impl CameraUniform {
    pub(crate) fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &CameraPosition, projection: &Projection) {
        self.view_proj = (projection.calc_matrix() * camera.calc_matrix()).to_cols_array_2d();
    }
}
