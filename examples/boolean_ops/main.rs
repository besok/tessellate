use glam::{Mat4, Vec3};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::Color;
use tessellate::mesh::parts::face::FaceType;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {

    let cube = Cube::create(Vertex::default(), 3.0, FaceType::Quad, Color::default());
    let pyramid = Pyramid::create(Vertex::default(), 5.0, 5.0, Color::default());

    let bool_op = pyramid.difference(cube)?;


    let meshes = vec![
        bool_op, 
    ];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}
