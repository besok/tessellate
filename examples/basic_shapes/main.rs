use glam::{Mat4, Quat, Vec3};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::parts::face::FaceType;
use tessellate::mesh::shape::cone::Cone;
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::cylinder::Cylinder;
use tessellate::mesh::shape::icosahedron::Icosahedron;
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::shape::ring::Ring;
use tessellate::mesh::shape::sphere::Sphere;
use tessellate::mesh::shape::torus::Torus;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{gpu, TessError, TessResult};
use tessellate::gpu::GpuOptions;
use tessellate::mesh::parts::vertex::Vertex;

fn main() -> TessResult<()> {
    let meshes = vec![
        cube_color_every_side()?.into(),
        green_sphere()?.into(),
        cone()?.into(),
        cyan_ico().into(),
        ring().into(),
        cylinder().into(),
        pyramid()?.into(),
        torus().into(),
    ];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}

fn torus() -> Torus {
    Torus::create(Vec3::new(0.0, -2.0, 2.0), 1.0, 0.5, 32, 32, Color::default())
}

fn cylinder() -> Cylinder {
    Cylinder::create([0.0, 0.0, 4.0], 1.0, 1.0, 3, Color::default())
}

fn green_sphere() -> TessResult<Sphere> {
    let mut sphere = Sphere::create_ico(Vertex::default(), 1.0, 3, RgbaColor::GREEN.into());
    sphere.transform(Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0)))?;
    Ok(sphere)
}

fn cone() -> Result<Cone, TessError> {
    let mut cone = Cone::default();

    cone.transform(Mat4::from_rotation_translation(
        Quat::from_rotation_x(30.0),
        Vec3::new(0.0, 1.0, 1.0),
    ))?;
    Ok(cone)
}

fn pyramid() -> Result<Pyramid, TessError> {
    let mut pyramid = Pyramid::default();
    pyramid.transform(Mat4::from_rotation_translation(
        Quat::from_rotation_x(0.0),
        Vec3::new(0.0, 1.0, -3.0),
    ))?;
    Ok(pyramid)
}

fn ring() -> Ring {
    Ring::create(Vec3::new(0.0, 4.0, 0.0), 1.0, 0.5, 4.0, 32, Color::default())
}

fn cyan_ico() -> Icosahedron {
    Icosahedron::create([3.0, 2.0, 1.0], 1.0, RgbaColor::CYAN)
}

fn cube_color_every_side() -> Result<Cube, TessError> {
    let mut cube = Cube::create(Vertex::default(), 1.0, FaceType::Quad, Color::default());
    let colors = cube
        .mesh()
        .faces()
        .iter()
        .map(|_| RgbaColor::random())
        .collect();

    cube.mesh_mut().set_color(Color::Face(colors));
    cube.transform(Mat4::from_translation(Vec3::new(-3.0, -3.0, 0.0)))?;
    Ok(cube)
}
