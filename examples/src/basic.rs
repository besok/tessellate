use glam::{Mat4, Vec3};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::RgbaColor;
use tessellate::mesh::parts::{BoundingBox, FaceType};
use tessellate::mesh::shape::cone::Cone;
use tessellate::{gpu, TessError};
use tessellate::mesh::HasMesh;
use tessellate::mesh::transform::Transform;
use crate::init_logger;

fn main() -> Result<(), TessError> {
    init_logger();

    // let ico = Icosahedron::create([3.0, 2.0, 1.0], 1.0, RgbaColor::CYAN);
    //
    // let mut ring = Ring::default();
    // ring.transform(Mat4::from_translation(Vec3::new(0.0, 4.0, 0.0)))?;
    //
    // let cylinder = Cylinder::create([0.0, 0.0, 4.0], 1.0, 1.0, 3, Color::default());
    //
    // let mut pyramid = Pyramid::default();
    // pyramid.transform(Mat4::from_rotation_translation(
    //     Quat::from_rotation_x(0.0),
    //     Vec3::new(0.0, 1.0, -3.0),
    // ))?;
    //
    // let mut torus = Torus::default();
    // torus.transform(Mat4::from_translation(Vec3::new(0.0, -2.0, 2.0)))?;
    //
    // let mut cube = Cube::create(Vertex::default(), 1.0, FaceType::Quad, Color::default());
    // let colors = cube
    //     .mesh()
    //     .faces()
    //     .iter()
    //     .map(|_| RgbaColor::random())
    //     .collect();
    // cube.mesh_mut().set_color(Color::Face(colors));
    // cube.transform(Mat4::from_translation(Vec3::new(-3.0, -3.0, 0.0)))?;
    //
    // let mut sphere = Sphere::create_ico(Vertex::default(), 1.0, 3, RgbaColor::GREEN.into());
    // let _ = sphere.transform(Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0)));
    //
    // let mut cone = Cone::default();
    //
    // cone.transform(Mat4::from_rotation_translation(
    //     Quat::from_rotation_x(30.0),
    //     Vec3::new(0.0, 1.0, 1.0),
    // ))?;
    //
    // let meshes = vec![
    //     cube.into(),
    //     sphere.into(),
    //     cone.into(),
    //     ico.into(),
    //     ring.into(),
    //     cylinder.into(),
    //     pyramid.into(),
    //     torus.into(),
    // ];

    let fig = Cone::default();
    let mesh = fig.mesh();

    let bbox:BoundingBox = mesh.bbox();
    let mut rect_cuboid = bbox.to_rect_cuboid(FaceType::default(), RgbaColor::default());

    rect_cuboid.transform(Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)))?;

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(vec![mesh.clone(), rect_cuboid.into()], camera)?)
}