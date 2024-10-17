use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;
/// Represents a Dini surface mesh.
#[derive(Debug, Clone)]
pub struct ConicSpiral {
    /// Parameter affecting the shape's radius.
    a: f32,
    /// Parameter affecting the shape's height.
    b: f32,
    /// Parameter affecting the shape's offset.
    c: f32,
    /// Parameter affecting the shape's frequency.
    n: f32,
    /// The mesh of the shape.
    mesh: Mesh,
}

impl Default for ConicSpiral {
    fn default() -> Self {
        ConicSpiral::create(Vertex::default(), 100, 0.2, 2.0, 0.1, 4.0, Color::default())
    }
}

impl ConicSpiral {
    /// Creates a new `ConicSpiral` shape with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center position of the shape.
    /// - `num_points`: The number of points to generate the shape.
    /// - `a`: Parameter affecting the shape's radius.
    /// - `b`: Parameter affecting the shape's height.
    /// - `c`: Parameter affecting the shape's offset.
    /// - `n`: Parameter affecting the shape's frequency.
    /// - `color`: The color of the shape.
    ///
    /// # Returns
    /// A new instance of `ConicSpiral`.
    pub fn create<V1, C>(
        center: V1,
        num_points: usize,
        a: f32,
        b: f32,
        c: f32,
        n: f32,
        color: C,
    ) -> Self
    where
        V1: Into<Vertex>,
        C: Into<Color>,
    {
        let center = center.into();
        let color = color.into();
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let inv2pi = 1.0 / (2.0 * PI);
        for i in 0..=num_points {
            let u = (i as f32 / num_points as f32) * PI * 2.0;
            for j in 0..=num_points {
                let v = (j as f32 / num_points as f32) * PI * 2.0;
                let cnv = (n * v).cos();
                let snv = (n * v).sin();
                let cu = u.cos();
                let su = u.sin();

                let x = a * (1.0 - v * inv2pi) * cnv * (1.0 + cu) + c * cnv;
                let y = a * (1.0 - v * inv2pi) * snv * (1.0 + cu) + c * snv;
                let z = b * v * inv2pi + a * (1.0 - v * inv2pi) * su;

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

        ConicSpiral {
            a,
            b,
            c,
            n,
            mesh: Mesh::from_vertices(vertices, faces, color),
        }
    }
}

impl Deref for ConicSpiral {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for ConicSpiral {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
