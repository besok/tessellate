use glam::{Mat4, Vec3};

use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::shape::parametric::conic_spiral::ConicSpiral;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};

fn main() -> TessResult<()> {
    let spiral = ConicSpiral::default();

    let edges = spiral.query().extract_boundary_edges()?;
    let mut mesh = Mesh::lines(edges, Color::Mesh(RgbaColor::GREEN));
    mesh.transform(Mat4::from_translation(Vec3::new(1., 0.0, 0.0)))?;

    let edges = spiral.query().extract_manifold_edges()?;
    let mut mesh2 = Mesh::lines(edges, Color::Mesh(RgbaColor::BLUE));
    mesh2.transform(Mat4::from_translation(Vec3::new(2., 0.0, 0.0)))?;

    let edges = spiral.query().extract_feature_edges(179.0)?;
    let mut mesh3 = Mesh::lines(edges, Color::Mesh(RgbaColor::RED));
    mesh3.transform(Mat4::from_translation(Vec3::new(3., 0.0, 0.0)))?;

    let edges = spiral.query().extract_non_manifold_edges()?;
    let mut mesh4 = Mesh::lines(edges, Color::Mesh(RgbaColor::RED));
    mesh4.transform(Mat4::from_translation(Vec3::new(4., 0.0, 0.0)))?;


    let meshes = vec![spiral.into(),   mesh, mesh2, mesh3, mesh4];
    let opts = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(-3.5, 2.0, 0.0)),
        LightOptions::new_position(Vec3::new(0.0, 3.0, 3.0)),
    );
    Ok(gpu::visualize(meshes, opts)?)
}
