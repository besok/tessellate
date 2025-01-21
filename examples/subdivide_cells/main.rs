use glam::{Mat4, Vec3};
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{files, gpu, mesh, TessResult};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};
use tessellate::mesh::attributes::Attributes;
use tessellate::mesh::parts::vertex::Vertex;

fn main() -> TessResult<()> {

    let ico = mesh::shape::icosahedron::Icosahedron::create(Vertex::default(), 0.2,  Attributes::default());
    let mut ico_b1 = ico.subdivide_by_loop(1)?;
    let mut ico_b2 = ico.subdivide_by_butterfly(1)?;
    let _ = ico_b1.transform(Mat4::from_translation(Vec3::new(-1., -0.5, 0.)))?;
    let _ = ico_b2.transform(Mat4::from_translation(Vec3::new(1., 0.5, 0.)))?;

    println!("Bunny vertices: {}", ico.vertices().len());
    println!("Bunny vertices: {}", ico_b1.vertices().len());
    println!("Bunny vertices: {}", ico_b2.vertices().len());

    let meshes = vec![
        ico.into(),
        ico_b1,
        ico_b2,
    ];

    let options = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(2., 2., 2.)),
        LightOptions::new_position(Vec3::new(3., 3., 0.))
            .with_show_source(true)
            .clone(),
    );

    Ok(gpu::visualize(meshes, options)?)
}
