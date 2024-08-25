use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::parts::polygon::Polygon;
use glam::Vec3;
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::Color;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh};
use tessellate::{gpu, poly,v, TessResult};

fn main() -> TessResult<()> {

    let mesh1 = Mesh::from_polygons(vec![poly!(0,0,0; 3,2,1 ; 2,2,2)], Color::default());
    let mesh2 = Mesh::from_polygons(vec![poly!(1,1,0; 0,0,0 ; 2,2,2)], Color::default());

    let meshes = vec![mesh1,mesh2];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}
