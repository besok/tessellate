use crate::mesh::parts::{Face, Vertex};
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use std::f32::consts::PI;
#[derive(Debug, Clone)]
pub struct Torus {
    center: Vertex,
    major_radius: f32,
    minor_radius: f32,
    mesh: Mesh,
}

impl Default for Torus {
    fn default() -> Self {
        Torus::create(Vertex::default(), 1.0, 0.5, 32, 16)
    }
}

impl Torus {
    pub fn create<V: Into<Vertex>>(
        center: V,
        major_radius: f32,
        minor_radius: f32,
        segments: usize,
        sides: usize,
    ) -> Self {
        let center = center.into();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for seg in 0..segments {
            let theta = (seg as f32) * 2.0 * PI / (segments as f32);
            for side in 0..sides {
                let phi = (side as f32) * 2.0 * PI / (sides as f32);
                let x = (major_radius + minor_radius * phi.cos()) * theta.cos();
                let y = (major_radius + minor_radius * phi.cos()) * theta.sin();
                let z = minor_radius * phi.sin();
                vertices.push(Vertex::new(x, y, z) + center)
            }
        }

        for seg in 0..segments {
            for side in 0..sides {
                let next_i = (seg + 1) % segments;
                let next_j = (side + 1) % sides;
                faces.push(Face::Quad(
                    seg * sides + side,
                    next_i * sides + side,
                    next_i * sides + next_j,
                    seg * sides + next_j,
                ));
            }
        }

        let mesh = Mesh::from_vertices(vertices, faces);

        Torus {
            major_radius,
            minor_radius,
            center,
            mesh,
        }
    }
}
impl HasMesh for Torus {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
