use std::ops::Deref;
use crate::mesh::attributes::Attributes;
use crate::mesh::Mesh;
use crate::mesh::parts::face::Face;
use crate::mesh::HasMesh;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Plane {
    center: Vertex,
    width: f32,
    height: f32,
    mesh: Mesh,
}
impl Deref for Plane {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl Plane {

    pub fn create<V,C>(center: V, width: f32, height: f32,attrs: C) -> Self
    where
        V:Into<Vertex>,
        C:Into<Attributes>
    {
        let center = center.into();
        let  attrs = attrs.into();
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

        let mesh = Mesh::from_vertices(vertices, faces,attrs);

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
        Plane::create(Vertex::default(), 1.0, 1.0,Color::default())
    }
}