use crate::mesh::Mesh;
use bytemuck::{Pod, Zeroable};
use rand::{Rng};
use std::mem;
use crate::mesh;

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


pub fn create_vertices(mesh: &Mesh) -> Vec<Vertex> {
    mesh.face_vertex()
        .unwrap_or_default()
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>()
        .to_vec()
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
