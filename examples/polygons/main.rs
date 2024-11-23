use glam::Vec3;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::gpu::options::GpuOptions;
use tessellate::mesh::attributes::Attributes;
use tessellate::mesh::material::Color;
use tessellate::mesh::parts::polygon::Polygon;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, poly, v, TessResult};

fn main() -> TessResult<()> {
    let mesh = Mesh::from_polygons(
        vec![
            poly!(-2.5, -2.5, 0.0; 2.5, -2.5, 0.0;0.0, 0.0, 5.0),
            poly!(2.5, -2.5, 0.0; 2.5, 2.5, 0.0; 0.0, 0.0, 5.0),
        ],
        Attributes::default(),
    );

    let meshes = vec![mesh ];

    Ok(gpu::visualize(meshes, GpuOptions::new_only_camera_pos(Vec3::new(-3.5, 0.0, 0.0)))?)
}
