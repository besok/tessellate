use glam::Vec3;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::Color;
use tessellate::mesh::parts::face::FaceType;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::torus::Torus;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{gpu, TessResult};

fn main() -> TessResult<()> {
    let cube = Cube::create(Vertex::default(), 2.0, FaceType::Quad, Color::default());
    let torus =  Torus::create(Vec3::new(0.0, 0., 0.), 2.0, 1.0, 32, 32, Color::default());

    let union = torus.intersection(cube)?;

    let meshes = vec![
        union
    ];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}


