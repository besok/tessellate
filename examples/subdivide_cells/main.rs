use glam::{Mat4, Vec3};
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{files, gpu, mesh, TessResult};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};
use tessellate::mesh::attributes::Attributes;
use tessellate::mesh::parts::vertex::Vertex;

fn main() -> TessResult<()> {

    // let bunny = files::ply::import_ply("examples/import_models/bunny.ply")?;
    let bunny = mesh::shape::icosahedron::Icosahedron::create(Vertex::default(), 0.2,  Attributes::default());
    let mut bunny_b = bunny.subdivide_by_loop(1)?;
    let _ = bunny_b.transform(Mat4::from_translation(Vec3::new(0.5, 0.5, 0.)))?;

    println!("Bunny vertices: {}", bunny.vertices().len());
    println!("Bunny vertices: {}", bunny_b.vertices().len());

    let meshes = vec![
        bunny.into(),
        bunny_b
    ];

    let options = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(1., 1., 1.)),
        LightOptions::new_position(Vec3::new(1., 1., 0.))
            .with_show_source(true)
            .clone(),
    );

    Ok(gpu::visualize(meshes, options)?)
}
