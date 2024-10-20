use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod processor;
mod vertex;

async fn run(
    meshes: Vec<Mesh>,
    camera: CameraPosition,
    settings: Settings,
) -> Result<(), GpuError> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    Ok(event_loop.run_app(&mut GpuProcessor::new(meshes, camera, settings))?)
}

pub fn visualize(
    meshes: Vec<Mesh>,
    camera: CameraPosition,
    settings: Settings,
) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, camera, settings))
}

#[derive(Clone, Debug)]
pub struct Settings {
    // if true, required_features: wgpu::Features::POLYGON_MODE_LINE and polygon_mode: wgpu::PolygonMode::Line
    pub only_lines: bool,
    pub topology: Topology,
}

impl Settings {
    pub fn new_topology(topology: Topology) -> Self {
        Settings {
            only_lines: Default::default(),
            topology,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            only_lines: false,
            topology: Topology::Triangles,
        }
    }
}
#[derive(Clone, Debug)]
pub enum Topology {
    Triangles,
    Lines,
    Points,
}

impl Topology {
    pub fn to_primitive_topology(&self) -> wgpu::PrimitiveTopology {
        match self {
            Topology::Triangles => wgpu::PrimitiveTopology::TriangleList,
            Topology::Lines => wgpu::PrimitiveTopology::LineList,
            Topology::Points => wgpu::PrimitiveTopology::PointList,
        }
    }
}
