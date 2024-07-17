use env_logger::Builder;
use glam::{EulerRot, Mat4, Quat, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::parts::{FaceType, Vertex};
use tessellate::mesh::shape::cone::Cone;
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::cuboid::rect_cuboid::RectCuboid;
use tessellate::mesh::shape::cylinder::Cylinder;
use tessellate::mesh::shape::grid::Grid;
use tessellate::mesh::shape::icosahedron::Icosahedron;
use tessellate::mesh::shape::plane::Plane;
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::shape::ring::Ring;
use tessellate::mesh::shape::sphere::Sphere;
use tessellate::mesh::shape::torus::Torus;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::Mesh;
use winit::error::EventLoopError;

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() -> Result<(), EventLoopError> {
    init_logger();

    let figure = Sphere::create_uv(Vertex::default(), 1.0, 32, 32);
    let figure = Icosahedron::create(Vertex::default(), 1.0);
    let figure = Ring::default();
    let figure = Cylinder::default();
    let figure = Pyramid::default();
    let figure = Grid::default();
    let mut figure = Plane::default();
    let mut figure = Torus::default();
    let mut figure1 = Cube::create(Vertex::default(), 1.0, FaceType::Triangle);
    let mut figure2 = Sphere::create_ico(Vertex::default(), 1.0, 3);
    let mut figure3 = Cone::default();

    let _ = figure2.transform(Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0)));
    let _ = figure3.transform(Mat4::from_rotation_translation(
        Quat::from_rotation_x(30.0),
        Vec3::new(0.0, 1.0, 1.0),
    ));

    let meshes = vec![figure1.into(), figure2.into(), figure3.into()];
    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    gpu::visualize(meshes, camera)
}
