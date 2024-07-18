use crate::mesh::parts::{Face, Vertex};
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use std::f32::consts::PI;
use crate::mesh::material::Color;

#[derive(Debug, Clone)]
pub struct Ring {
    center: Vertex,
    inner_radius: f32,
    outer_radius: f32,
    height: f32,
    mesh: Mesh,
}

impl Default for Ring {
    fn default() -> Self {
        Ring::create(Vertex::default(), 1.0, 0.5, 1.0, 32, Color::default())
    }
}

impl Ring {
    pub fn create<V: Into<Vertex>>(
        center: V,
        inner_radius: f32,
        outer_radius: f32,
        height: f32,
        segments: usize,
        color: Color
    ) -> Self {
        let center = center.into();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        // Generate vertices for the outer and inner radius at both top and bottom heights
        for seg in 0..segments {
            let angle = (seg as f32) * 2.0 * PI / (segments as f32);
            let outer_x = outer_radius * angle.cos();
            let outer_y = outer_radius * angle.sin();
            let inner_x = inner_radius * angle.cos();
            let inner_y = inner_radius * angle.sin();
            // Bottom vertices
            vertices.push(Vertex::new(
                center.x + outer_x,
                center.y + outer_y,
                center.z - height / 2.0,
            )); // Outer vertex bottom
            vertices.push(Vertex::new(
                center.x + inner_x,
                center.y + inner_y,
                center.z - height / 2.0,
            )); // Inner vertex bottom
                // Top vertices
            vertices.push(Vertex::new(
                center.x + outer_x,
                center.y + outer_y,
                center.z + height / 2.0,
            )); // Outer vertex top
            vertices.push(Vertex::new(
                center.x + inner_x,
                center.y + inner_y,
                center.z + height / 2.0,
            )); // Inner vertex top
        }

        // Generate faces for the outer surface, inner surface, top, and bottom
        for i in 0..segments {
            let next_i = (i + 1) % segments;
            let current_outer_bottom = i * 4;
            let current_inner_bottom = i * 4 + 1;
            let current_outer_top = i * 4 + 2;
            let current_inner_top = i * 4 + 3;
            let next_outer_bottom = next_i * 4;
            let next_inner_bottom = next_i * 4 + 1;
            let next_outer_top = next_i * 4 + 2;
            let next_inner_top = next_i * 4 + 3;

            faces.push(Face::Quad(
                current_outer_bottom,
                next_outer_bottom,
                next_outer_top,
                current_outer_top,
            ));
            faces.push(Face::Quad(
                current_inner_bottom,
                current_inner_top,
                next_inner_top,
                next_inner_bottom,
            ));
            faces.push(Face::Quad(
                current_outer_top,
                next_outer_top,
                next_inner_top,
                current_inner_top,
            ));
            faces.push(Face::Quad(
                current_outer_bottom,
                current_inner_bottom,
                next_inner_bottom,
                next_outer_bottom,
            ));
        }

        let mesh = Mesh::from_vertices(vertices, faces,color);

        Ring {
            center,
            inner_radius,
            outer_radius,
            height,
            mesh,
        }
    }
}
impl HasMesh for Ring {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
