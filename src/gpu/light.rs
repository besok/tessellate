use crate::gpu::GpuOptions;
use crate::mesh::material::RgbaColor;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::util::DeviceExt;
use egui_wgpu::wgpu::{BindGroupLayout, Device, SurfaceConfiguration};
use glam::{Quat, Vec3};

pub struct Light {
    light_uniform: LightUniform,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
    light_bind_layout: BindGroupLayout,
}

impl Light {
    pub fn init(device: &Device, gnu_options: &GpuOptions) -> Self {
        let light_uniform =
            LightUniform::new(&gnu_options.light.position, &gnu_options.light.color);
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

    pub fn update_position(&mut self) {
        self.light_uniform.update_position();
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
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding: u32,
    color: [f32; 3],
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding2: u32,
}

impl LightUniform {
    pub fn new(position: &Vec3, color: &RgbaColor) -> Self {
        let c = color.0;
        Self {
            position: (*position).into(),
            _padding: 0,
            color: [c[0] as f32, c[1] as f32, c[2] as f32],
            _padding2: 0,
        }
    }

    pub fn update_position(&mut self) {
        let old_pos: Vec3 = self.position.into();
        self.position = (Quat::from_axis_angle(Vec3::Y, 1.0_f32.to_radians()) * old_pos).into();
    }

    pub fn position(&self) -> Vec3 {
        self.position.into()
    }
}
