use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;
use crate::mesh::attributes::Attributes;

/// Bour's minimal surface
/// https://en.wikipedia.org/wiki/Bour%27s_minimal_surface
#[derive(Debug, Clone)]
pub struct Bour {
    theta_steps: usize,
    theta_max: f32,
    r_steps: usize,
    r_min: f32,
    r_max: f32,
    mesh: Mesh,
}

impl Default for Bour {
    fn default() -> Self {
        Bour::create(Vertex::default(), 50, 4.0, 50, 0., 3., Attributes::default())
    }
}

impl Bour {
    /// Creates a new instance of Bour's minimal surface.
    ///
    /// # Parameters
    /// - `center`: The center vertex of the surface.
    /// - `theta_steps`: The number of steps for the theta parameter.
    /// - `theta_max`: The maximum value for the theta parameter.
    /// - `r_steps`: The number of steps for the radius parameter.
    /// - `r_min`: The minimum value for the radius parameter.
    /// - `r_max`: The maximum value for the radius parameter.
    /// - `color`: The color of the surface.
    pub fn create<V1, C>(
        center: V1,
        theta_steps: usize,
        theta_max: f32,
        r_steps: usize,
        r_min: f32,
        r_max: f32,
        attrs: C,
    ) -> Self
    where
        V1: Into<Vertex>,
        C: Into<Attributes>,
    {
        let center = center.into();
        let  attrs = attrs.into();
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for i in 0..=theta_steps {
            let th = (i as f32 / theta_steps as f32) * theta_max * PI; // u from 0 to 2π
            for j in 0..=r_steps {
                let r = r_min + (j as f32 / r_steps as f32) * (r_max - r_min); // v from 0 to 1

                // Parametric equations for Bour's minimal surface:
                let x = r * th.cos() - 0.5 * r.powi(2) * (2. * th).cos();
                let y = -r * th.sin() - 0.5 * r.powi(2) * (2. * th).sin();
                let z = (4. / 3.) * r.powf(1.5) * (1.5 * th).cos();

                vertices.push(center + Vertex::new(x, y, z));
            }
        }

        for i in 0..theta_steps {
            for j in 0..r_steps {
                let a = i * (r_steps + 1) + j;
                let b = a + r_steps + 1;
                let c = a + 1;
                let d = b + 1;

                faces.push(Face::new3(a, b, c));
                faces.push(Face::new3(b, d, c));
            }
        }

        Bour {
            theta_steps,
            theta_max,
            r_steps,
            r_min,
            r_max,
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for Bour {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Bour {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
