use env_logger::Builder;
use glam::{EulerRot, Mat4, Quat, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};
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
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{gpu, TessError};

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

    let opts = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(-3.5, 2.0, 0.0)),
        LightOptions::new_position(Vec3::new(0.0, 3.0, 3.0)),
    );
    Ok(gpu::visualize(vec![mesh.clone(), rect_cuboid.into()], opts)?)
}
