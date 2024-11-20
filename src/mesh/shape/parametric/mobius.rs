use core::f32::consts::PI;
use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::ops::Deref;
use crate::mesh::attributes::Attributes;

/// Represents a Möbius Strip mesh.
#[derive(Debug, Clone)]
pub struct MobiusStrip {
    width: f32,
    length: f32,
    twists: f32,
    mesh: Mesh,
}

impl Default for MobiusStrip {
    fn default() -> Self {
        MobiusStrip::create(Vertex::default(), 1.0, 5.0, 3.0, 100, Color::default())
    }
}

impl MobiusStrip {
    /// Creates a new `MobiusStrip` shape with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center position of the shape.
    /// - `width`: The width of the strip.
    /// - `length`: The length of the strip.
    /// - `twists`: The number of twists in the strip.
    /// - `num_points`: The number of points to generate the shape.
    /// - `color`: The color of the shape.
    ///
    /// # Returns
    /// A new instance of `MobiusStrip`.
    pub fn create<V1, C>(
        center: V1,
        width: f32,
        length: f32,
        twists: f32,
        num_points: usize,
        attrs: C,
    ) -> Self
    where
        V1: Into<Vertex>,
        C: Into<Attributes>,
    {
        let center = center.into();
        let attrs = attrs.into();
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for i in 0..=num_points {
            let t = i as f32 / num_points as f32 * 2.0 * PI;
            for j in 0..=1 {
                let s = j as f32 * width - width / 2.0;
                let x = (length + s * (twists * t / 2.0).cos()) * t.cos();
                let y = (length + s * (twists * t / 2.0).cos()) * t.sin();
                let z = s * (twists * t / 2.0).sin();

                vertices.push(center + Vertex::new(x, y, z));
            }
        }

        for i in 0..num_points {
            let a = i * 2;
            let b = a + 1;
            let c = (a + 2) % (2 * (num_points + 1));
            let d = (b + 2) % (2 * (num_points + 1));
            faces.push(Face::new3(a, b, c));
            faces.push(Face::new3(b, c, d));
        }

        MobiusStrip {
            width,
            length,
            twists,
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for MobiusStrip {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for MobiusStrip {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
