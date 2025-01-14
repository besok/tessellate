use glam::{Mat4, Vec3};
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{files, gpu, TessResult};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};

fn main() -> TessResult<()> {
    let bunny = files::ply::import_ply("examples/import_models/bunny.ply")?;
    let mut bunny_new = bunny.subdivide().by_butterfly()?;

    let _ = bunny_new.transform(Mat4::from_translation(Vec3::new(0.3, 0.3, 0.)))?;

    let meshes = vec![
        bunny,
        bunny_new
    ];

    let options = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(1., 1., 1.)),
        LightOptions::new_position(Vec3::new(0., 0., 0.))
            .with_show_source(true)
            .clone(),
    );

    Ok(gpu::visualize(meshes, options)?)
}
