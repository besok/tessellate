use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::ops::Deref;
use crate::mesh::attributes::Attributes;

#[derive(Debug, Clone)]
pub struct Supertoroid {
    /// The radius from the center to the middle of the ring of the supertoroid.
    ring_rad: f32,
    /// The radius of the cross section of ring of the supertoroid.
    cross_section_rad: f32,
    /// The scaling factor for the x-axis.
    x_rad: f32,
    /// The scaling factor for the y-axis.
    y_rad: f32,
    /// The scaling factor for the z-axis.
    z_rad: f32,
    /// The shape of the torus ring.
    n1: f32,
    /// The shape of the cross section of the torus ring.
    n2: f32,
    mesh: Mesh,
}

impl Default for Supertoroid {
    fn default() -> Self {
        Supertoroid::create(
            Vertex::default(),
            1.0,
            0.5,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            100,
            Color::default(),
        )
    }
}

impl Supertoroid {
    /// Creates a new `Supertoroid` with the specified parameters.
    ///
    /// # Parameters
    /// - `center`: The center of the supertoroid.
    /// - `ring_rad`: The radius from the center to the middle of the ring of the supertoroid.
    /// - `cross_section_rad`: The radius of the cross section of the ring of the supertoroid.
    /// - `x_rad`: The scaling factor for the x-axis.
    /// - `y_rad`: The scaling factor for the y-axis.
    /// - `z_rad`: The scaling factor for the z-axis.
    /// - `n1`: The shape of the torus ring.
    /// - `n2`: The shape of the cross section of the torus ring.
    /// - `u_segments`: The number of segments along the u-axis.
    /// - `v_segments`: The number of segments along the v-axis.
    /// - `color`: The color of the supertoroid.
    ///
    /// # Returns
    /// A new `Supertoroid` instance.
    pub fn create<V1, C>(
        center: V1,
        ring_rad: f32,
        cross_section_rad: f32,
        x_rad: f32,
        y_rad: f32,
        z_rad: f32,
        n1: f32,
        n2: f32,
        steps: usize,
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

        let step_u = 2.0 * std::f32::consts::PI / steps as f32;
        let step_v = 2.0 * std::f32::consts::PI / steps as f32;

        // Generate vertices
        for i in 0..=steps {
            let u = i as f32 * step_u;
            for j in 0..=steps {
                let v = j as f32 * step_v;

                // Parametric coordinates of the supertoroid
                let cos_u = u.cos();
                let sin_u = u.sin();
                let cos_v = v.cos();
                let sin_v = v.sin();

                // Use absolute powers and sign functions to control sharpness
                let x = x_rad
                    * (ring_rad + cross_section_rad * cos_v.signum() * cos_v.abs().powf(n1))
                    * cos_u.signum()
                    * cos_u.abs().powf(n2);
                let y = y_rad
                    * (ring_rad + cross_section_rad * cos_v.signum() * cos_v.abs().powf(n1))
                    * sin_u.signum()
                    * sin_u.abs().powf(n2);
                let z = z_rad * cross_section_rad * sin_v.signum() * sin_v.abs().powf(n1);

                vertices.push(Vertex::new(
                    center.x + x,
                    center.y + y,
                    center.z + z,
                ));
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

        Supertoroid {
            ring_rad,
            cross_section_rad,
            x_rad,
            y_rad,
            z_rad,
            n1,
            n2,
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for Supertoroid {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Supertoroid {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
