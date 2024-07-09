use bytemuck::{Pod, Zeroable};
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

fn vertex(p12: ([i8; 3], [i8; 3])) -> Vertex {
    let (p, c) = p12;
    Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
    }
}

pub fn create_vertices() -> Vec<Vertex> {
    let pos = cube_positions();
    let col = cube_colors();

    (0..pos.len())
        .into_iter()
        .map(|i| (pos[i], col[i]))
        .map(vertex)
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
#[rustfmt::skip]
pub fn cube_positions() -> Vec<[i8; 3]> {
    [
        // front (0, 0, 1)
        [-1, -1,  1], [1, -1,  1], [-1,  1,  1], [-1,  1,  1], [ 1, -1,  1], [ 1,  1,  1],

        // right (1, 0, 0)
        [ 1, -1,  1], [1, -1, -1], [ 1,  1,  1], [ 1,  1,  1], [ 1, -1, -1], [ 1,  1, -1],

        // back (0, 0, -1)
        [ 1, -1, -1], [-1, -1, -1], [1,  1, -1], [ 1,  1, -1], [-1, -1, -1], [-1,  1, -1],

        // left (-1, 0, 0)
        [-1, -1, -1], [-1, -1,  1], [-1,  1, -1], [-1,  1, -1], [-1, -1,  1], [-1,  1,  1],

        // top (0, 1, 0)
        [-1,  1,  1], [ 1,  1,  1], [-1,  1, -1], [-1,  1, -1], [ 1,  1,  1], [ 1,  1, -1],

        // bottom (0, -1, 0)
        [-1, -1, -1], [ 1, -1, -1], [-1, -1,  1], [-1, -1,  1], [ 1, -1, -1], [ 1, -1,  1],
    ].to_vec()
}
#[rustfmt::skip]
pub fn cube_colors() -> Vec<[i8; 3]> {
    [
        // front - blue
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],

        // right - red
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],

        // back - yellow
        [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0],

        // left - aqua
        [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1],

        // top - green
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],

        // bottom - fuchsia
        [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1],
    ].to_vec()
}
