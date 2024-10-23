use glam::{Mat4, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {
    let pyramid  = Pyramid::default() ;

    let poly_centers = pyramid.query().extract_poly_centers()?;
    let edge_centers = pyramid.query().extract_edge_centers()?;

    let mesh1 = Mesh::cloud(poly_centers,0.02, Color::Mesh(RgbaColor::GREEN));
    let mesh2 = Mesh::cloud(edge_centers, 0.02, Color::Mesh(RgbaColor::RED));


    let meshes = vec![mesh1, mesh2, pyramid.into()];
    let camera = CameraPosition::new(Vec3::new(-2.5, 0.0, 0.0), 0.0, 0.0);

    Ok(gpu::visualize(meshes, camera)?)
}
