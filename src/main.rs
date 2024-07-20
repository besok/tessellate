use env_logger::Builder;
use glam::{EulerRot, Mat4, Quat, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
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
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessError};
use winit::error::EventLoopError;

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}
fn main() -> Result<(), TessError> {
    init_logger();

    let ico = Icosahedron::create([3.0, 2.0, 1.0], 1.0, RgbaColor::CYAN);

    let mut ring = Ring::default();
    ring.transform(Mat4::from_translation(Vec3::new(0.0, 4.0, 0.0)))?;

    let cylinder = Cylinder::create([0.0, 0.0, 4.0], 1.0, 1.0, 3, Default::default());

    let mut pyramid = Pyramid::default();
    pyramid.transform(Mat4::from_rotation_translation(
        Quat::from_rotation_x(0.0),
        Vec3::new(0.0, 1.0, -3.0),
    ))?;

    let mut torus = Torus::default();
    torus.transform(Mat4::from_translation(Vec3::new(0.0, -2.0, 2.0)))?;

    let mut cube = Cube::create(Vertex::default(), 1.0, FaceType::Quad, Default::default());
    let colors = cube
        .mesh()
        .faces()
        .iter()
        .map(|_| RgbaColor::random())
        .collect();
    cube.mesh_mut().set_color(Color::Face(colors));
    cube.transform(Mat4::from_translation(Vec3::new(-3.0, -3.0, 0.0)))?;

    let mut sphere = Sphere::create_ico(Vertex::default(), 1.0, 3, RgbaColor::GREEN.into());
    let _ = sphere.transform(Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0)));

    let mut cone = Cone::default();

    cone.transform(Mat4::from_rotation_translation(
        Quat::from_rotation_x(30.0),
        Vec3::new(0.0, 1.0, 1.0),
    ))?;

    let meshes = vec![
        cube.into(),
        sphere.into(),
        cone.into(),
        ico.into(),
        ring.into(),
        cylinder.into(),
        pyramid.into(),
        torus.into(),
    ];
    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}
