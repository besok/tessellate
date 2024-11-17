use glam::Vec3;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::gpu::options::GpuOptions;
use tessellate::mesh::material::Color;
use tessellate::mesh::parts::face::Face;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {
    let vertices = vec![
        Vertex::new(0.0, 0.0, 0.0),
        Vertex::new(1.0, 0.0, 0.0),
        Vertex::new(1.0, 1.0, 0.0),
        Vertex::new(0.0, 1.0, 0.0),
    ];
    let faces = vec![
        Face::from((0, 1, 2)),
        Face::from((0, 2, 3)),
        Face::from((0, 3, 1)),
    ];
    let mesh = Mesh::from_vertices(vertices, faces, Color::default());

    let meshes = vec![mesh];

    Ok(gpu::visualize(meshes, GpuOptions::new_only_camera_pos(Vec3::new(-3.5, 0.0, 0.0)))?)
}
