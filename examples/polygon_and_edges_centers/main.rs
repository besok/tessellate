use glam::{Mat4, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};

fn main() -> TessResult<()> {
    let pyramid  = Pyramid::default() ;

    let poly_centers = pyramid.query().extract_poly_centers()?;
    let edge_centers = pyramid.query().extract_edge_centers()?;

    let mesh1 = Mesh::cloud(poly_centers,2, Color::Mesh(RgbaColor::GREEN).into());
    let mesh2 = Mesh::cloud(edge_centers, 2, Color::Mesh(RgbaColor::RED).into());


    let meshes = vec![mesh1, mesh2, pyramid.into()];
    let opts = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(-3.5, 2.0, 0.0)),
        LightOptions::new_position(Vec3::new(0.0, 2.0, 3.0)),
    );
    Ok(gpu::visualize(meshes, opts)?)
}
