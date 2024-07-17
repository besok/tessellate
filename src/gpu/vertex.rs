use crate::mesh;
use crate::mesh::parts::{Face, FaceType};
use crate::mesh::{Mesh, MeshError};
use bytemuck::{Pod, Zeroable};
use rand::Rng;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl From<&mesh::parts::Vertex> for Vertex {
    fn from(value: &mesh::parts::Vertex) -> Self {
        let mut rng = rand::thread_rng();
        let color = [
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        ];
        let p = value.flatten();
        Vertex {
            position: [p[0], p[1], p[2], 1.0],
            color,
        }
    }
}
impl TryFrom<&Mesh> for Vec<Vertex> {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> Result<Self, Self::Error> {
        Ok(mesh
            .faces()
            .iter()
            .flat_map(face_to_vertex3)
            .map(|i| mesh.get_v(i))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>())
    }
}

pub fn face_to_vertex3(face: &Face) -> Vec<usize> {
    match face {
        Face::Triangle(a, b, c) => vec![*a, *b, *c],
        Face::Quad(a, b, c, d) => vec![*a, *b, *c, *a, *c, *d],
    }
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4];
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
