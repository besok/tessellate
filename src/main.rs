use env_logger::Builder;
use glam::{EulerRot, Mat4, Quat, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::parts::bbox::BoundingBox;
use tessellate::mesh::parts::face::FaceType;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::query::bsp::BSPTree;
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

    let fig = Cone::default();
    let mesh = fig.mesh();

    let bbox: BoundingBox = mesh.aabb();
    let mut rect_cuboid = bbox.to_rect_cuboid(FaceType::default(), RgbaColor::default());

    rect_cuboid.transform(Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)))?;

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(vec![mesh.clone(), rect_cuboid.into()], camera,  )?)
}
