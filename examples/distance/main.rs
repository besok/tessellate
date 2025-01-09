use glam::{Mat4, Vec3};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};
use tessellate::mesh::distance::{distance_between_surfaces, distance_to_surface};
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::shape::torus::Torus;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{gpu, v, TessResult};

fn main() -> TessResult<()> {
    let torus = Torus::default();
    let mut pyramid = Pyramid::default();
    pyramid.transform(Mat4::from_translation(Vec3::new(2.0, 2.0, 2.0)))?;

    let torus_centroid = torus.centroid()?;
    let pyramid_centroid = pyramid.centroid()?;

    println!("Distances");
    println!("  Centroids = {}", torus_centroid.distance(&pyramid_centroid));
    println!("  Torus centroid and pyramid = {}", distance_to_surface(&torus_centroid, &pyramid)?);
    println!("  Pyramid centroid and torus = {}", distance_to_surface(&pyramid_centroid, &torus)?);
    println!("  Surfaces = {}", distance_between_surfaces(&pyramid, &torus)?);

    Ok(gpu::visualize(vec![torus.mesh().clone(), pyramid.mesh().clone()], GpuOptions::default())?)
}
