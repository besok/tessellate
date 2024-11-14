use crate::mesh;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::util::DeviceExt;
use egui_wgpu::wgpu::{BindGroupLayout, Device};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

impl MaterialUniform {
    pub fn new(m: &mesh::material::Material) -> Self {
        Self {
            ambient: m.ambient(),
            diffuse: m.diffuse(),
            specular: m.specular(),
            shininess: m.shininess(),
        }
    }
}

pub struct Material {
    material_uniform: MaterialUniform,
    material_buffer: wgpu::Buffer,
    material_bind_group: wgpu::BindGroup,
    material_bind_layout: BindGroupLayout,
}

impl Material {
    pub fn create_bind_group_layout(device: &Device) -> BindGroupLayout {
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
        })
    }

    pub fn init(device: &Device, mesh_material: &mesh::material::Material) -> Self {
        let material_uniform = MaterialUniform::new(mesh_material);

        let material_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Material Buffer"),
            contents: bytemuck::cast_slice(&[material_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let material_bind_layout = Self::create_bind_group_layout(device);
        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &material_bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: material_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            material_uniform,
            material_buffer,
            material_bind_group,
            material_bind_layout,
        }
    }

    pub fn material_bind_group(&self) -> &wgpu::BindGroup {
        &self.material_bind_group
    }

    pub fn material_bind_layout(&self) -> &BindGroupLayout {
        &self.material_bind_layout
    }
}
