use crate::mesh::Mesh;
use crate::mesh::parts::{Face, Vertex};
use crate::mesh::HasMesh;
#[derive(Debug, Clone)]
pub struct Plane {
    center: Vertex,
    width: f32,
    height: f32,
    mesh: Mesh,
}

impl Plane {

    pub fn create<V: Into<Vertex>>(center: V, width: f32, height: f32) -> Self {
        let center = center.into();
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();


        vertices.push(Vertex::new(center.x, center.y - half_width, center.z - half_height));
        vertices.push(Vertex::new(center.x, center.y + half_width, center.z - half_height));
        vertices.push(Vertex::new(center.x, center.y + half_width, center.z + half_height));
        vertices.push(Vertex::new(center.x, center.y - half_width, center.z + half_height));


        faces.push(Face::Triangle(0, 1, 2));
        faces.push(Face::Triangle(2, 3, 0));

        let mesh = Mesh::from_vertices(vertices, faces);

        Self {
            center,
            width,
            height,
            mesh,
        }
    }
}

impl HasMesh for Plane {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane::create(Vertex::default(), 1.0, 1.0)
    }
}