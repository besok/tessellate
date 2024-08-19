use glam::{Mat4, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::Color;
use tessellate::mesh::query::MeshQuery;
use tessellate::mesh::shape::cone::Cone;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {
    let mut mesh: Mesh = Cone::default().into();
    let kdtree = MeshQuery::new(&mesh).try_sskd_tree(None, None)?;

    mesh.transform(Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)))?;

    let meshes = vec![kdtree.to_mesh(Color::default()), mesh];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}
