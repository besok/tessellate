use crate::mesh::attributes::Attributes;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;

/// Represents an Ellipsoid shape with radii along the x, y, and z axes.
#[derive(Debug, Clone)]
pub struct SuperEllipsoid {
    x_rad: f32,
    y_rad: f32,
    z_rad: f32,
    n1: f32,
    n2: f32,
    mesh: Mesh,
}

impl Default for SuperEllipsoid {
    fn default() -> Self {
        SuperEllipsoid::create(Vertex::default(), 50, 1.0, 2.0, 3.0, 4.0, 5.0, Attributes::default())
    }
}

impl SuperEllipsoid {
    /// Creates a new `SuperEllipsoid with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center of the ellipsoid.
    /// - `steps`: The number of steps for the parametric grid.
    /// - `x_rad`: The radius along the x-axis.
    /// - `y_rad`: The radius along the y-axis.
    /// - `z_rad`: The radius along the z-axis.
    /// - `n1`: The exponent for the latitude.
    /// - `n2`: The exponent for the longitude.
    /// - `color`: The color of the ellipsoid.
    ///
    /// # Returns
    /// A new `SuperEllipsoid` instance.
    pub fn create<V1, C>(
        center: V1,
        steps: usize,
        x_rad: f32,
        y_rad: f32,
        z_rad: f32,
        n1: f32,
        n2: f32,
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

        let step_u = 2.0 * PI / steps as f32;
        let step_v = PI / steps as f32;

        for i in 0..=steps {
            let u = i as f32 * step_u;
            for j in 0..=steps {
                let v = j as f32 * step_v - PI / 2.0;

                let cos_u = u.cos();
                let sin_u = u.sin();
                let cos_v = v.cos();
                let sin_v = v.sin();

                let x = x_rad
                    * cos_v.abs().powf(2.0 / n1)
                    * cos_u.abs().powf(2.0 / n2)
                    * cos_u.signum();
                let y = y_rad
                    * cos_v.abs().powf(2.0 / n1)
                    * sin_u.abs().powf(2.0 / n2)
                    * sin_u.signum();
                let z = z_rad * sin_v.abs().powf(2.0 / n1) * sin_v.signum();

                vertices.push(center + Vertex::new(x, y, z));
            }
        }

        // Generate faces (quads) by connecting neighboring vertices in the parametric grid
        for i in 0..steps {
            for j in 0..steps {
                // Calculate vertex indices in the 2D grid (u-v space)
                let v00 = i * (steps + 1) + j; // Current vertex
                let v01 = i * (steps + 1) + (j + 1); // Next vertex in v-direction
                let v10 = (i + 1) * (steps + 1) + j; // Next vertex in u-direction
                let v11 = (i + 1) * (steps + 1) + (j + 1); // Next in both directions (u and v)

                // Create a quad face from these four vertices
                faces.push((v00, v01, v11, v10));
            }
        }

        SuperEllipsoid {
            x_rad,
            y_rad,
            z_rad,
            n1,
            n2,
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for SuperEllipsoid {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for SuperEllipsoid {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
