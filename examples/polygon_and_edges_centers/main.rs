use glam::{Mat4, Quat, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::gpu::{Settings, Topology};
use tessellate::mesh::material::Color;
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessError, TessResult};

fn main() -> TessResult<()> {
    let pyramid: Mesh = Pyramid::default().into();

    let poly_centers = pyramid.query().extract_poly_centers()?;
    let edge_centers = pyramid.query().extract_edge_centers()?;

    let mesh1 = Mesh::only_vertices(poly_centers, Color::default());
    let mesh2 = Mesh::only_vertices(edge_centers, Color::default());

    let meshes = vec![mesh1, mesh2];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera, Settings::new_topology(Topology::Points))?)
}
