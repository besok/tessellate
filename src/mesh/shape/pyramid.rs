use std::ops::Deref;
use crate::mesh::attributes::Attributes;
use crate::mesh::parts::face::Face;
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Pyramid {
    center: Vertex,
    size: f32,
    height: f32,
    mesh: Mesh,
}
impl Deref for Pyramid {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl Default for Pyramid {
    fn default() -> Self {
        Pyramid::create(Vertex::default(), 1.0, 3.0,Attributes::default())
    }
}
impl Pyramid {
    pub fn create<V,C>(center: V, size: f32, height: f32,attrs: C) -> Self
    where V:Into<Vertex>,
          C:Into<Attributes>
    {
        let center = center.into();
        let attrs = attrs.into();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let half_size = size / 2.0;
        vertices.push(Vertex::new(center.x - half_size, center.y - half_size, center.z));
        vertices.push(Vertex::new(center.x + half_size, center.y - half_size, center.z));
        vertices.push(Vertex::new(center.x + half_size, center.y + half_size, center.z));
        vertices.push(Vertex::new(center.x - half_size, center.y + half_size, center.z));
        let tip_index = vertices.len();

        vertices.push(Vertex::new(center.x, center.y, center.z + height));

        faces.push(Face::Triangle(0, 1, tip_index));
        faces.push(Face::Triangle(1, 2, tip_index));
        faces.push(Face::Triangle(2, 3, tip_index));
        faces.push(Face::Triangle(3, 0, tip_index));

        let base_center = Vertex::new(center.x, center.y, center.z);
        let base_center_index = vertices.len();
        vertices.push(base_center);
        faces.push(Face::Triangle(base_center_index, 0, 1));
        faces.push(Face::Triangle(base_center_index, 1, 2));
        faces.push(Face::Triangle(base_center_index, 2, 3));
        faces.push(Face::Triangle(base_center_index, 3, 0));

        let mesh = Mesh::from_vertices(vertices, faces,attrs);
        Self {
            center,
            size,
            height,
            mesh,
        }
    }
}

impl HasMesh for Pyramid {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
