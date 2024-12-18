use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use std::ops::Deref;
use crate::mesh::attributes::Attributes;

#[derive(Debug, Clone)]
pub struct Beam {
    start: Vertex,
    diam: f32,
    end: Vertex,
    mesh: Mesh,
}

impl Deref for Beam {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl Default for Beam {
    fn default() -> Self {
        Beam::create(Vertex::default(), Vertex::new(1.0,1.0,1.0), 0.01,  Attributes::default())
    }
}

impl Beam {
    pub fn create<V1,V2, C>(start: V1, end: V2, diam:f32, attrs: C) -> Self
    where
        V1: Into<Vertex>,
        V2: Into<Vertex>,
        C: Into<Attributes>,
    {
        let start = start.into();
        let end = end.into();
        let attrs = attrs.into();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let width = diam;
        let height = diam;

        // Calculate the vertices for the beam
        vertices.push(start);
        vertices.push(Vertex::new(start.x + width, start.y, start.z));
        vertices.push(Vertex::new(start.x, start.y + height, start.z));
        vertices.push(Vertex::new(start.x + width, start.y + height, start.z));
        vertices.push(end);
        vertices.push(Vertex::new(end.x + width, end.y, end.z));
        vertices.push(Vertex::new(end.x, end.y + height, end.z));
        vertices.push(Vertex::new(end.x + width, end.y + height, end.z));

        // Define the faces of the beam
        faces.push(Face::Quad(0, 1, 5, 4)); // Front face
        faces.push(Face::Quad(2, 3, 7, 6)); // Back face
        faces.push(Face::Quad(0, 2, 6, 4)); // Left face
        faces.push(Face::Quad(1, 3, 7, 5)); // Right face
        faces.push(Face::Quad(0, 1, 3, 2)); // Bottom face
        faces.push(Face::Quad(4, 5, 7, 6)); // Top face

        let mesh = Mesh::from_vertices(vertices, faces, attrs);
        Beam { start, end,diam, mesh }
    }
}

impl HasMesh for Beam {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
