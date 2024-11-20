use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{HasMesh, Mesh};
use std::f32::consts::PI;
use std::ops::Deref;
use crate::mesh::attributes::Attributes;

/// A Bohemian Dome is a parametric shape that is a generalization of a torus.
#[derive(Debug, Clone)]
pub struct BohemianDome {
    a: f32,
    b: f32,
    c: f32,
    u_steps: usize,
    v_steps: usize,
    mesh: Mesh,
}

impl Default for BohemianDome {
    fn default() -> Self {
        BohemianDome::create(Vertex::default(), 0.5, 1.5, 1.0, 50, 50, Attributes::default())
    }
}

impl BohemianDome {
    /// Creates a new `BohemianDome` with the given parameters.
    ///
    /// # Parameters
    /// - `center`: The center of the dome.
    /// - `a`: The scaling factor for the x-axis.
    /// - `b`: The scaling factor for the y-axis.
    /// - `c`: The scaling factor for the z-axis.
    /// - `u_steps`: The number of segments along the u-axis.
    /// - `v_steps`: The number of segments along the v-axis.
    /// - `color`: The color of the dome.
    ///
    /// # Returns
    /// A new `BohemianDome` instance.
    ///
    /// # Example
    /// ```rust
    /// use tessellate::mesh::attributes::Attributes;
    /// use tessellate::mesh::material::Color;
    /// use tessellate::mesh::parts::vertex::Vertex;
    /// use tessellate::mesh::shape::parametric::bohemian_dome::BohemianDome;
    /// let dome = BohemianDome::create(
    ///     Vertex::default(),
    ///     1.0,
    ///     1.0,
    ///     1.0,
    ///     20,
    ///     20,
    ///     Attributes::default(),
    /// );
    /// ```
    pub fn create<V1, C>(
        center: V1,
        a: f32,
        b: f32,
        c: f32,
        u_steps: usize,
        v_steps: usize,
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

        for i in 0..=u_steps {
            let u = (i as f32 / u_steps as f32) * 2.0 * PI; // u from 0 to 2π
            for j in 0..=v_steps {
                let v = (j as f32 / v_steps as f32) * 2.0 * PI; // v from 0 to π

                // Parametric equations for the dome (Bohemian Dome assumption):
                let x = a * u.cos();
                let y = b * v.cos() + a * u.sin();
                let z = c * v.sin();

                // Add the point to the list
                vertices.push(center + Vertex::new(x, y, z));
            }
        }

        for i in 0..u_steps {
            for j in 0..v_steps {
                let a = i * (v_steps + 1) + j;
                let b = a + v_steps + 1;
                let c = a + 1;
                let d = b + 1;

                faces.push(Face::new3(a, b, c));
                faces.push(Face::new3(b, d, c));
            }
        }

        BohemianDome {
            a,
            b,
            c,
            u_steps,
            v_steps,
            mesh: Mesh::from_vertices(vertices, faces, attrs),
        }
    }
}

impl Deref for BohemianDome {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for BohemianDome {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
