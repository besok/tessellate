use glam::{Mat4, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::shape::parametric::conic_spiral::ConicSpiral;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {
    let spiral = ConicSpiral::default();

    let edges = spiral.query().extract_boundary_edges()?;
    let mut mesh = Mesh::lines(edges, Color::Mesh(RgbaColor::GREEN));
    mesh.transform(Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)))?;

    let meshes = vec![spiral.into(), mesh];
    let camera = CameraPosition::new(Vec3::new(-2.5, 0.0, 0.0), 0.0, 0.0);

    Ok(gpu::visualize(meshes, camera)?)
}
