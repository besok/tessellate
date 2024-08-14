use std::f32::consts::PI;
use std::ops::Deref;
use crate::mesh::Mesh;
use crate::mesh::parts::face::Face;
use crate::mesh::HasMesh;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::shape::cone::Cone;

#[derive(Debug, Clone)]
pub struct Cylinder {
    center: Vertex,
    radius: f32,
    height: f32,
    mesh: Mesh,
}
impl Deref for Cylinder {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl Default for Cylinder{
    fn default() -> Self {
        Cylinder::create(Vertex::default(), 1.0, 2.0, 32, Color::default())
    }
}
impl Cylinder {
    pub fn create<V,C>(
        center: V,
        radius: f32,
        height: f32,
        segments: usize,
        color: C
    ) -> Self
    where V:Into<Vertex>,
          C:Into<Color>
    {
        let center = center.into();
        let color = color.into();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();


        // Bottom center vertex
        let bottom_center_index = vertices.len();
        vertices.push(Vertex::new(center.x, center.y, center.z - height / 2.0));

        // Generate vertices for the bottom circle
        for i in 0..segments {
            let angle = (i as f32) * 2.0 * PI / (segments as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(Vertex::new(center.x + x, center.y + y, center.z - height / 2.0));
        }

        let top_center_index = vertices.len();
        vertices.push(Vertex::new(center.x, center.y, center.z + height / 2.0));

        for seg in 0..segments {
            let angle = (seg as f32) * 2.0 * PI / (segments as f32);
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(Vertex::new(center.x + x, center.y + y, center.z + height / 2.0));
        }

        for seg in 0..segments {
            let next_seg = (seg + 1) % segments;
            let bottom_i = bottom_center_index + 1 + seg;
            let top_i = top_center_index + 1 + seg;
            let next_bottom_i = bottom_center_index + 1 + next_seg;
            let next_top_i = top_center_index + 1 + next_seg;
            faces.push(Face::Quad(bottom_i, next_bottom_i, next_top_i, top_i));
        }

        for seg in 0..segments {
            let top_seg = (seg + 1) % segments;
            let bot_seg = bottom_center_index + 1 + seg;
            let next_bot_seg = bottom_center_index + 1 + top_seg;
            faces.push(Face::Triangle(bottom_center_index, bot_seg, next_bot_seg));
        }

        // Generate faces for the top circle
        for seg in 0..segments {
            let next_seg = (seg + 1) % segments;
            let top_seg = top_center_index + 1 + seg;
            let next_top_seg = top_center_index + 1 + next_seg;
            faces.push(Face::Triangle(top_center_index, top_seg, next_top_seg));
        }

        let mesh = Mesh::from_vertices(vertices, faces,color);
        Cylinder {
            center,
            radius,
            height,
            mesh,
        }
    }
}

impl HasMesh for Cylinder {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}