use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::material::RgbaColor;
use crate::mesh::Mesh;
use glam::Vec3;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod gui;
mod light;
pub mod material;
mod processor;
mod vertex;

async fn run(
    meshes: Vec<Mesh>,
    camera: CameraPosition,
    options: GpuOptions,
) -> Result<(), GpuError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    Ok(event_loop.run_app(&mut GpuProcessor::new(meshes, camera, options))?)
}

pub fn visualize(meshes: Vec<Mesh>, camera: CameraPosition) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, camera, GpuOptions::default()))
}

pub fn visualize_with(
    meshes: Vec<Mesh>,
    camera: CameraPosition,
    options: GpuOptions,
) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, camera, options))
}

#[derive(Debug, Clone)]
pub struct GpuOptions {
    camera_speed: f32,
    camera_sensitivity: f32,
    light: LightOptions,
}
#[derive(Debug, Clone)]
pub struct LightOptions {
    position: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
}

impl LightOptions {
    pub fn new(
        position: Vec3,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
        }
    }
    pub fn new_only_pos(position: Vec3) -> Self {
        Self {
            position,
            ambient: Vec3::new(0.1, 0.1, 0.1),
            diffuse: Vec3::new(0.5, 0.5, 0.5),
            specular: Vec3::new(0.5, 0.5, 0.5),
        }
    }


    pub fn with_position(&mut self, position: Vec3) -> &mut Self {
        self.position = position;
        self
    }

    pub fn with_ambient(&mut self, ambient: Vec3) -> &mut Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(&mut self, diffuse: Vec3) -> &mut Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(&mut self, specular: Vec3) -> &mut Self {
        self.specular = specular;
        self
    }
}

impl GpuOptions {
    pub fn new(camera_speed: f32, camera_sensitivity: f32, light: LightOptions) -> Self {
        Self {
            camera_speed,
            camera_sensitivity,
            light,
        }
    }
    pub fn with_camera_speed(&mut self, camera_speed: f32) -> &mut Self {
        self.camera_speed = camera_speed;
        self
    }

    pub fn with_camera_sensitivity(&mut self, camera_sensitivity: f32) -> &mut Self {
        self.camera_sensitivity = camera_sensitivity;
        self
    }

    pub fn with_light(&mut self, light: LightOptions) -> &mut Self {
        self.light = light;
        self
    }
}

impl Default for GpuOptions {
    fn default() -> Self {
        Self {
            camera_speed: 0.1,
            camera_sensitivity: 0.001,
            light: LightOptions::new_only_pos(Vec3::new(2.0, 2.0, 2.0)),
        }
    }
}
