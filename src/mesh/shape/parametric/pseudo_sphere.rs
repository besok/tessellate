use crate::mesh::material::Color; 
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;
use crate::mesh::attributes::Attributes;
use crate::mesh::parts::face::Face;

/// Represents an Ellipsoid shape with radii along the x, y, and z axes.
#[derive(Debug, Clone)]
pub struct Pseudosphere {

    mesh: Mesh,
}

impl Default for Pseudosphere {
    fn default() -> Self {
        Pseudosphere::create(Vertex::default(), 100,   Color::default())
    }
}

impl Pseudosphere {
    /// Creates a new `Pseudosphere` with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center of the pseudosphere.
    /// - `steps`: The number of steps for the parametric grid.
    /// - `color`: The color of the pseudosphere.
    ///
    /// # Returns
    /// A new `Pseudosphere` instance.
    pub fn create<V1, C>(
        center: V1,
        steps: usize,
        attrs: C,
    ) -> Self
    where
        V1: Into<Vertex>,
        C: Into<Attributes>,
    {
        let center = center.into();
        let attrs = attrs.into() ;
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let step_u = 2.0 * PI / steps as f32;
        let step_v = PI / steps as f32;

        for i in 0..=steps {
            let u = i as f32 * step_u;
            for j in 0..=steps {
                let v = j as f32 * step_v;

                let x = u.cos() * v.sin();
                let y = u.sin() * v.sin();
                let z = v.cos() + (0.5 * v).tan().ln_1p();

                vertices.push(center + Vertex::new(x, y, z));
            }
        }

        for i in 0..steps {
            for j in 0..steps {
                let v00 = i * (steps + 1) + j;
                let v01 = i * (steps + 1) + (j + 1);
                let v10 = (i + 1) * (steps + 1) + j;
                let v11 = (i + 1) * (steps + 1) + (j + 1);

                faces.push(Face::new4(v00, v01, v11, v10));
            }
        }

        Pseudosphere {
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for Pseudosphere {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Pseudosphere {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
