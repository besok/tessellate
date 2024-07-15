use env_logger::Builder;
use glam::Vec3;
use log::{info, LevelFilter};
use tessellate::gpu;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::parts::{FaceType, Vertex};
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::cuboid::rect_cuboid::RectCuboid;
use winit::error::EventLoopError;
use tessellate::mesh::shape::cone::Cone;
use tessellate::mesh::shape::icosahedron::Icosahedron;
use tessellate::mesh::shape::ring::Ring;
use tessellate::mesh::shape::sphere::Sphere;
use tessellate::mesh::shape::torus::Torus;

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() -> Result<(), EventLoopError> {
    init_logger();

    // let figure = Cube::create(Vertex::default(), 1.0,FaceType::Quad );
    // let figure = Sphere::create_uv(Vertex::default(), 1.0, 32, 32);
    // let figure = Sphere::create_ico(Vertex::default(), 1.0, 6);
    // let figure = Icosahedron::create(Vertex::default(), 1.0);
    // let figure = Torus::default();
    // let figure = Ring::default();
    let figure = Cone::default();

    let meshes = vec![figure.into()];
    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    gpu::visualize(meshes, camera)
}
