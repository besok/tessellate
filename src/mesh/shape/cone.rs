use crate::mesh::parts::{Face, Vertex};
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use std::f32::consts::PI;
use crate::mesh::material::Color;

#[derive(Debug, Clone)]
pub struct Cone {
    center: Vertex,
    radius: f32,
    height: f32,
    mesh: Mesh,
}

impl Default for Cone {
    fn default() -> Self {
        Cone::create(Vertex::default(), 1.0, 2.0, 32,Color::default())
    }
}

impl Cone {
    pub fn create<V: Into<Vertex>>(
        center: V,
        radius: f32,
        height: f32,
        segments: usize,
        color: Color
    ) -> Self {
        let center = center.into();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let tip = Vertex::new(center.x, center.y, center.z + height);
        vertices.push(tip);


        for seg in 0..segments {
            let angle = (seg as f32) * 2.0 * PI / (segments as f32);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            vertices.push(Vertex::new(x, y, center.z));
        }

        for seg in 1..=segments {
            faces.push(Face::Triangle(0, seg, (seg + 1) % segments));
        }

        for seg in 1..segments-1 {
            faces.push(Face::Triangle(1, seg + 1, seg + 2));
        }

        let mesh = Mesh::from_vertices(vertices, faces,color);

        Cone {
            center,
            radius,
            height,
            mesh,
        }
    }
}
impl HasMesh for Cone {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
