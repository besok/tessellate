use glam::{Mat4, Quat, Vec3};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};

use tessellate::mesh::shape::cone::Cone;

use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessError, TessResult};
use tessellate::mesh::bool::sskdtree::SSKDTree;

fn main() -> TessResult<()> {
    let mut mesh:Mesh = Cone::default().into();
    let kdtree = SSKDTree::try_from_mesh(&mesh, None,None)?;

    mesh.transform(Mat4::from_translation(
        Vec3::new(2.0, 0.0, 0.0),
    ))?;

    let meshes = vec![
        kdtree.to_mesh(Color::default()), mesh
    ];


    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}

