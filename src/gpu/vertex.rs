#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color: [f32; 3],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub(crate) fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    // far side (0.0, 0.0, 1.0)
    Vertex { position: [-1.0, -1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, -1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, 1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, 1.0, 1.0], color: [0.5, 0.0, 0.5] },
    // near side (0.0, 0.0, -1.0)
    Vertex { position: [-1.0, 1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, 1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, -1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, -1.0, -1.0], color: [0.5, 0.0, 0.5] },
    // right side (1.0, 0.0, 0.0)
    Vertex { position: [1.0, -1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, 1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, 1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, -1.0, 1.0], color: [0.5, 0.0, 0.5] },
    // left side (-1.0, 0.0, 0.0)
    Vertex { position: [-1.0, -1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, 1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, 1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, -1.0, -1.0], color: [0.5, 0.0, 0.5] },
    // top (0.0, 1.0, 0.0)
    Vertex { position: [1.0, 1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, 1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, 1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, 1.0, 1.0], color: [0.5, 0.0, 0.5] },
// bottom (0.0, -1.0, 0.0)
    Vertex { position: [1.0, -1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, -1.0, 1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-1.0, -1.0, -1.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [1.0, -1.0, -1.0], color: [0.5, 0.0, 0.5] },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // far
    4, 5, 6, 6, 7, 4, // near
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // top
    20, 21, 22, 22, 23, 20, // bottom
];
