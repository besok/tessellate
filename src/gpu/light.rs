use crate::gpu::options::GpuOptions;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::util::DeviceExt;
use egui_wgpu::wgpu::{BindGroupLayout, Device};
use glam::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct IsAffectedByLightUniform {
    pub is_affected_by_light: u32,
}

pub struct Light {
    light_uniform: LightUniform,

    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
    light_bind_layout: BindGroupLayout,
}

impl Light {
    pub fn init(device: &Device, gnu_options: &GpuOptions) -> Self {
        let light_uniform:LightUniform = gnu_options.into();
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light VB"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });
        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            light_uniform,
            light_buffer,
            light_bind_group,
            light_bind_layout: light_bind_group_layout,
        }
    }

    pub fn light_bind_group(&self) -> &wgpu::BindGroup {
        &self.light_bind_group
    }

    pub fn light_bind_layout(&self) -> &BindGroupLayout {
        &self.light_bind_layout
    }

    pub fn light_uniform(&self) -> &LightUniform {
        &self.light_uniform
    }

    pub fn light_buffer(&self) -> &wgpu::Buffer {
        &self.light_buffer
    }

    pub fn position(&self) -> Vec3 {
        self.light_uniform.position()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    position: [f32; 3],
    _padding1: u32,
    ambient: [f32; 3],
    _padding2: u32,
    diffuse: [f32; 3],
    _padding3: u32,
    specular: [f32; 3],
    _padding4: u32,
}

impl From<&GpuOptions> for LightUniform {
    fn from(options: &GpuOptions) -> Self {
        LightUniform::new(
            &options.light_opts().position().into(),
            &options.light_opts().ambient().into(),
            &options.light_opts().diffuse().into(),
            &options.light_opts().specular().into(),
        )
    }
}

impl LightUniform {
    pub fn new (position: &Vec3, ambient: &Vec3, diffuse: &Vec3, specular: &Vec3) -> Self {
        Self {
            position: (*position).into(),
            _padding1: 0,
            ambient: (*ambient).into(),
            _padding2: 0,
            diffuse: (*diffuse).into(),
            _padding3: 0,
            specular: (*specular).into(),
            _padding4: 0,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position.into()
    }
}
