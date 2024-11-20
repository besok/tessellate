use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;
use crate::mesh::attributes::Attributes;

/// Represents a Dini surface mesh.
#[derive(Debug, Clone)]
pub struct Dini {
    a: f32,
    b: f32,
    mesh: Mesh,
}

impl Default for Dini {
    fn default() -> Self {
        Dini::create(Vertex::default(), 100, 1., 1., Color::default())
    }
}

impl Dini {
    /// Creates a new `Dini` shape with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center position of the shape.
    /// - `num_points`: The number of points to generate the shape.
    /// - `a`: Parameter `a` for the Dini surface equation.
    /// - `b`: Parameter `b` for the Dini surface equation.
    /// - `color`: The color of the shape.
    ///
    /// # Returns
    /// A new instance of `Dini`.
    pub fn create<V1, C>(center: V1, num_points: usize, a: f32, b: f32, attrs: C) -> Self
    where
        V1: Into<Vertex>,
        C: Into<Attributes>,
    {
        let center = center.into();
        let  attrs = attrs.into();
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for i in 0..=num_points {
            let u = (i as f32 / num_points as f32) * 4. * PI;
            for j in 0..=num_points {
                let v = 0.1 + (j as f32 / num_points as f32) * (1.5 - 0.1);

                let x = a * u.cos() * v.sin();
                let y = a * u.sin() * v.sin();
                let z =  a * (v.cos()  + (v/2.0).tan().ln()) + b * u;

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

        Dini {
            a,b,
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for Dini {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Dini {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
