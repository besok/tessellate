use glam::{Mat4, Vec3};
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh, MeshError};
use tessellate::{files, gpu, TessError, TessResult};
use tobj::LoadOptions;

fn main() -> TessResult<()> {
    let cow = files::obj::import_obj("examples/import_models/cow.obj", &LoadOptions::default())?;
    let axle = files::ply::import_ply("examples/import_models/axle_shaft.ply")?;

    let meshes = vec![cow];

    Ok(gpu::visualize(meshes, Vec3::default().into())?)
}
