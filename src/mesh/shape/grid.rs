use std::ops::Deref;
use crate::mesh::attributes::Attributes;
use crate::mesh::Mesh;
use crate::mesh::parts::face::Face;
use crate::mesh::HasMesh;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Grid {
    rows: usize,
    columns: usize,
    spacing: f32,
    mesh: Mesh,
}
impl Deref for Grid {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl Grid {
    pub fn create(rows: usize, columns: usize, spacing: f32,attrs: Attributes) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for row in 0..rows {
            for col in 0..columns {
                let x = col as f32 * spacing;
                let z = row as f32 * spacing;
                vertices.push(Vertex::new(x, 0.0, z));
            }
        }

        for row in 0..rows - 1 {
            for col in 0..columns - 1 {
                let current = row * columns + col;
                let right = current + 1;
                let below = current + columns;
                let diagonal = below + 1;

                // First triangle
                faces.push(Face::Triangle(current, right, diagonal));
                // Second triangle
                faces.push(Face::Triangle(current, diagonal, below));
            }
        }

        let mesh = Mesh::from_vertices(vertices, faces,attrs);

        Self {
            rows,
            columns,
            spacing,
            mesh,
        }
    }
}

impl HasMesh for Grid {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid::create(10, 10, 1.0,Default::default())
    }
}