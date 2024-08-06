use crate::mesh::material::{Color, RgbaColor};
use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshError, parts};
use bytemuck::{Pod, Zeroable};
use std::iter::zip;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl Vertex {
    fn from(v: &parts::vertex::Vertex, color: &RgbaColor) -> Self {
        let v = v.flatten();
        Vertex {
            position: [v[0], v[1], v[2], 1.0],
            color: color.clone().into(),
        }
    }
}

impl TryFrom<&Mesh> for Vec<Vertex> {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> Result<Self, Self::Error> {
        match mesh.color() {
            Color::Face(fs) => {
                let faces = mesh.faces();
                faces_check(fs, faces)?;
                let mut vertices = Vec::new();
                for (col, face) in zip(fs.into_iter(), faces.into_iter()) {
                    let vs = face_to_vertex3(face)
                        .into_iter()
                        .map(|i| mesh.get(i))
                        .collect::<Result<Vec<_>, _>>()?;
                    vertices.extend(vs.into_iter().map(|v| Vertex::from(v, col)))
                }
                Ok(vertices)
            }
            Color::Mesh(m) => Ok(mesh
                .faces()
                .iter()
                .flat_map(face_to_vertex3)
                .map(|i| mesh.get(i))
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|v| Vertex::from(v, m))
                .collect::<Vec<_>>()),
            Color::Func(f) => {
                let mut vertices = Vec::new();
                for face in mesh.faces().into_iter() {
                    let vs = face_to_vertex3(face)
                        .into_iter()
                        .map(|i| mesh.get(i))
                        .collect::<Result<Vec<_>, _>>()?;
                    vertices.extend(
                        vs.into_iter()
                            .enumerate()
                            .map(|(i, v)| Vertex::from(v, &f(v, i))),
                    )
                }
                Ok(vertices)
            }
            Color::Vertex(colors) => {
                let mut vertices = Vec::new();
                for face in mesh.faces().into_iter() {
                    let vs = face_to_vertex3(face)
                        .into_iter()
                        .map(|i| mesh.get(i))
                        .collect::<Result<Vec<_>, _>>()?;
                    vertices.extend(
                        zip(vs.into_iter(), colors.into_iter()).map(|(v, c)| Vertex::from(v, c)),
                    )
                }
                Ok(vertices)
            }
        }
    }
}

fn faces_check(fs: &Vec<RgbaColor>, faces: &Vec<Face>) -> Result<(), MeshError> {
    if fs.len() != faces.len() {
        Err(MeshError::InvalidFaceType(format!(
            "Face color count {} does not match face count {}",
            fs.len(),
            faces.len()
        )))
    } else {
        Ok(())
    }
}
fn vertices_check(
    vs: &Vec<RgbaColor>,
    vertices: &Vec<parts::vertex::Vertex>,
) -> Result<(), MeshError> {
    if vs.len() != vertices.len() {
        return Err(MeshError::InvalidIndex(format!(
            "Vertex color count {} does not match vertex count {}",
            vs.len(),
            vertices.len()
        )));
    } else {
        Ok(())
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
