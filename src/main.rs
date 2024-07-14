use env_logger::Builder;
use glam::Vec3;
use log::{info, LevelFilter};
use tessellate::gpu;
use tessellate::mesh::parts::{FaceType, Vertex};
use tessellate::mesh::primitives::cuboid::Cube;
use winit::error::EventLoopError;
use tessellate::gpu::camera::position::CameraPosition;

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() -> Result<(), EventLoopError> {
    init_logger();


    let meshes = vec![Cube::from_center(Vertex::default(), 1.0, FaceType::Triangle).into()];
    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    gpu::visualize(meshes,camera)
}
