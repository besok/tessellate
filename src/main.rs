use env_logger::Builder;
use log::{info, LevelFilter};
use tessellate::gpu;
use tessellate::mesh::parts::{FaceType, Vertex};
use tessellate::mesh::primitives::cuboid::Cube;
use winit::error::EventLoopError;

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() -> Result<(), EventLoopError> {
    init_logger();

    gpu::visualize(vec![Cube::from_center(Vertex::default(), 1.0, FaceType::Triangle).into()])
}
