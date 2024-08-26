use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::parts::polygon::Polygon;
use glam::Vec3;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::Color;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, poly,v, TessResult};
use tessellate::mesh::bool::Ray;
use tessellate::mesh::shape::beam::Beam;

fn main() -> TessResult<()> {
    let p1 = poly!(1, 0, 0; 0, 1, 0; 0, 0, 1);
    let mesh1 = Mesh::from_polygons(vec![p1.clone()], Color::default());
    let mesh2 = Mesh::from_polygons(vec![poly!(0, 0, 0; 0, 2, 0; 1, 1, 1)], Color::default());

    let ray = Ray::from_poly(&p1, &Vertex::new(2.0, 2.0, 2.0))?;

    let beam = ray.to_beam(0.01, Color::default(),);

    let meshes = vec![mesh1,beam.into(), mesh2];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}
