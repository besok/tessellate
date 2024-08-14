use std::ops::Deref;
use crate::mesh::parts::face::Face;
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::shape::grid::Grid;

#[derive(Debug, Clone)]
pub struct Icosahedron {
    center: Vertex,
    scale: f32,
    mesh: Mesh,
}

impl HasMesh for Icosahedron {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}
impl Deref for Icosahedron {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl Icosahedron {
    pub fn create<V,C>(center: V, size: f32,color: C) -> Self
    where V: Into<Vertex>,
          C: Into<Color>
    {
        let center = center.into();
        let color = color.into();
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
        let scale = size / phi.sqrt();
        Self {
            center,
            scale,
            mesh: Mesh::from_vertices(vertices(center, phi, scale), faces(),color),
        }
    }
}
fn vertices(center: Vertex, phi: f32, scale: f32) -> Vec<Vertex> {
    [
        [-1., phi, 0.],
        [1., phi, 0.],
        [-1., -phi, 0.],
        [1., -phi, 0.],
        [0., -1., phi],
        [0., 1., phi],
        [0., -1., -phi],
        [0., 1., -phi],
        [phi, 0., -1.],
        [phi, 0., 1.],
        [-phi, 0., -1.],
        [-phi, 0., 1.],
    ]
    .into_iter()
    .map(Into::into)
    .map(|v: Vertex| v * scale + center)
    .collect()
}
#[rustfmt::skip]
fn faces() -> Vec<Face> {
    vec![
        (0, 11, 5), (0, 5, 1), (0, 1, 7), (0, 7, 10), (0, 10, 11),
        (1, 5, 9), (5, 11, 4), (11, 10, 2), (10, 7, 6), (7, 1, 8),
        (3, 9, 4), (3, 4, 2), (3, 2, 6), (3, 6, 8), (3, 8, 9),
        (4, 9, 5), (2, 4, 11), (6, 2, 10), (8, 6, 7), (9, 8, 1),
    ].into_iter().map(Into::into).collect()
}
