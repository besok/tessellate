use env_logger::Builder;
use glam::{Mat4, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::parts::{FaceType, Vertex};
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::cuboid::rect_cuboid::RectCuboid;
use winit::error::EventLoopError;
use tessellate::mesh::Mesh;
use tessellate::mesh::shape::cone::Cone;
use tessellate::mesh::shape::cylinder::Cylinder;
use tessellate::mesh::shape::grid::Grid;
use tessellate::mesh::shape::icosahedron::Icosahedron;
use tessellate::mesh::shape::plane::Plane;
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::shape::ring::Ring;
use tessellate::mesh::shape::sphere::Sphere;
use tessellate::mesh::shape::torus::Torus;
use tessellate::mesh::transform::Transform;

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() -> Result<(), EventLoopError> {
    init_logger();

    let figure = Sphere::create_uv(Vertex::default(), 1.0, 32, 32);
    let figure = Sphere::create_ico(Vertex::default(), 1.0, 6);
    let figure = Icosahedron::create(Vertex::default(), 1.0);
    let figure = Ring::default();
    let figure = Cylinder::default();
    let figure = Pyramid::default();
    let figure = Grid::default();
    let mut figure = Plane::default();
    let mut figure = Torus::default();
    let mut figure = Cone::default();
    let mut figure = Cube::create(Vertex::default(), 1.0,FaceType::Quad );

    let _ = figure.transform(Mat4::from_rotation_z(10.));

    let meshes = vec![figure.into()];
    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    gpu::visualize(meshes, camera)
}
