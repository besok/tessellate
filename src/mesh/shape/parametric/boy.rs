use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;

/// Represents a Boy surface mesh.
#[derive(Debug, Clone)]
pub struct Boy {
    mesh: Mesh,
}

impl Default for Boy {
    fn default() -> Self {
        Boy::create(Vertex::default(), 100, Color::default())
    }
}

impl Boy {
    /// Creates a new `Boy` shape with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center position of the shape.
    /// - `num_points`: The number of points to generate the shape.
    /// - `color`: The color of the shape.
    ///
    /// # Returns
    /// A new instance of `Boy`.
    pub fn create<V1, C>(center: V1, num_points: usize, color: C) -> Self
    where
        V1: Into<Vertex>,
        C: Into<Color>,
    {
        let center = center.into();
        let color = color.into();
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for i in 0..=num_points {
            let u =  (i as f32 / num_points as f32) * PI;
            for j in 0..=num_points {
                let v = (j as f32 / num_points as f32) * PI;

                let x = (2.0 / 3.0)
                    * (u.cos() * (2.0 * v).cos() + 2.0_f32.sqrt() * u.sin() * v.cos())
                    * u.cos() / (2.0_f32.sqrt() - (2.0 * u).sin() * (3.0 * v).sin());
                let y = (2.0 / 3.0)
                    * (u.cos() * (2.0 * v).sin() - 2.0_f32.sqrt() * u.sin() * v.sin())
                    * u.cos() / (2.0_f32.sqrt() - (2.0 * u).sin() * (3.0 * v).sin());
                let z = -2.0_f32.sqrt() * u.cos() * u.cos() / (2.0_f32.sqrt() - (2.0 * u).sin() * (3.0 * v).sin());


                vertices.push(center + Vertex::new(x, y, z));
            }
        }

        for i in 0..num_points {
            for j in 0..num_points {
                let a = i * (num_points + 1) + j;
                let b = a + num_points + 1;
                let c = a + 1;
                let d = b + 1;
                faces.push(Face::new3(a, b, c));
                faces.push(Face::new3(b, c, d));
            }
        }

        Boy {
            mesh: Mesh::from_vertices(vertices, faces, color),
        }
    }
}

impl Deref for Boy {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Boy {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
