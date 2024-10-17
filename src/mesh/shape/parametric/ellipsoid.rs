use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::ops::Deref;
/// Represents an Ellipsoid shape with radii along the x, y, and z axes.
#[derive(Debug, Clone)]
pub struct Ellipsoid {
    x_rad: f32,
    y_rad: f32,
    z_rad: f32,
    mesh: Mesh,
}

impl Default for Ellipsoid {
    fn default() -> Self {
        Ellipsoid::create(Vertex::default(),32, 3.0,4.0, 5.0, Color::default())
    }
}

impl Ellipsoid {
    /// Creates a new `Ellipsoid` with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center of the ellipsoid.
    /// - `steps`: The number of steps for the parametric grid.
    /// - `x_rad`: The radius along the x-axis.
    /// - `y_rad`: The radius along the y-axis.
    /// - `z_rad`: The radius along the z-axis.
    /// - `color`: The color of the ellipsoid.
    ///
    /// # Returns
    /// A new `Ellipsoid` instance.
    pub fn create<V1, C>(
        center: V1,
        steps: usize,
        x_rad: f32,
        y_rad: f32,
        z_rad: f32,
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

        let step_u = 2.0 * std::f32::consts::PI / steps as f32;
        let step_v = 2.0 * std::f32::consts::PI / steps as f32;

        // Generate vertices
        for i in 0..=steps {
            let u = i as f32 * step_u;
            for j in 0..=steps {
                let v = j as f32 * step_v;

                let x = x_rad * u.cos() * v.sin();
                let y = y_rad * u.sin() * v.sin();
                let z = z_rad * v.cos();

                vertices.push(Vertex::new(center.x + x, center.y + y, center.z + z));
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

        Ellipsoid {
            x_rad,
            y_rad,
            z_rad,
            mesh: Mesh::from_vertices(vertices, faces, color),
        }
    }
}

impl Deref for Ellipsoid {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Ellipsoid {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
