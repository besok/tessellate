use glam::{Mat4, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::gpu::options::GpuOptions;
use tessellate::mesh::material::Color;
use tessellate::mesh::shape::cylinder::Cylinder;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {
    let mut mesh: Mesh = Cylinder::default().into();
    let kdtree = &mesh.query().try_sskd_tree(None, None)?;

    mesh.transform(Mat4::from_translation(Vec3::new(2.0, 0.0, 0.0)))?;

    let mut meshes = vec![kdtree.to_mesh(Color::default()), mesh];
    // meshes.extend(kdtree.aabb_to_mesh());

    Ok(gpu::visualize(meshes, GpuOptions::new_only_camera_pos(Vec3::new(-3.5, 0.0, 0.0)))?)
}
