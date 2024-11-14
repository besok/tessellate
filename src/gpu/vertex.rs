use crate::mesh::attributes::MeshType;
use crate::mesh::material::{Color, RgbaColor};
use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::face::Face;
use crate::mesh::{parts, Mesh, MeshError};
use bytemuck::{Pod, Zeroable};
use egui_wgpu::wgpu;
use glam::Vec3;
use std::iter::zip;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct GpuInstance {
    position: [f32; 4],
}

impl From<GpuVertex> for GpuInstance {
    fn from(v: GpuVertex) -> Self {
        GpuInstance {
            position: v.position,
        }
    }
}

impl GpuInstance {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0=>Float32x4];
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GpuInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct GpuVertex {
    position: [f32; 4],
    color: [f32; 4],
    normal: [f32; 4],
}

impl GpuVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4];
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<GpuVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl GpuVertex {
    fn from(v: &parts::vertex::Vertex, color: &RgbaColor, n: &Vec3) -> Self {
        let v = v.flatten();
        GpuVertex {
            position: [v[0], v[1], v[2], 1.0],
            color: color.clone().into(),
            normal: [n[0], n[1], n[2], 1.0],
        }
    }
}

impl TryFrom<&Mesh> for Vec<GpuVertex> {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> Result<Self, Self::Error> {
        let normals = mesh.try_normals()?;
        match mesh.attributes().mesh_type() {
            MeshType::Polygons => match mesh.color() {
                Color::Face(fs) => {
                    let faces = mesh.faces();
                    faces_check(fs, faces)?;
                    let mut vertices = Vec::new();
                    for (col, face) in zip(fs.into_iter(), faces.into_iter()) {
                        let normal = normals.get_face_normal(face)?;
                        let vs = face_to_vertex3(face)
                            .into_iter()
                            .map(|i| mesh.get(i))
                            .collect::<Result<Vec<_>, _>>()?;
                        vertices.extend(vs.into_iter().map(|v| GpuVertex::from(v, col, normal)))
                    }
                    Ok(vertices)
                }
                Color::Mesh(m) => {
                    let mut vertices = Vec::new();
                    for face in mesh.faces().iter() {
                        let normal = normals.get_face_normal(face)?;
                        for idx in face_to_vertex3(face) {
                            let v = mesh.get(idx)?;
                            vertices.push(GpuVertex::from(v, m, normal));
                        }
                    }
                    Ok(vertices)
                }
                Color::Func(f) => {
                    let mut vertices = Vec::new();
                    for face in mesh.faces().into_iter() {
                        let normal = normals.get_face_normal(face)?;
                        for idx in face_to_vertex3(face) {
                            let v = mesh.get(idx)?;
                            vertices.push(GpuVertex::from(v, &f(v, idx), normal));
                        }
                    }
                    Ok(vertices)
                }
                Color::Vertex(colors) => {
                    let mut vertices = Vec::new();
                    for face in mesh.faces().into_iter() {
                        let normal = normals.get_face_normal(face)?;
                        for idx in face_to_vertex3(face) {
                            let v = mesh.get(idx)?;
                            vertices.push(GpuVertex::from(
                                v,
                                &colors.get(idx).ok_or(MeshError::idx_vertex(idx))?.clone(),
                                normal,
                            ));
                        }
                    }
                    Ok(vertices)
                }
                Color::Line(_) => Err(MeshError::InvalidFaceType(
                    "Line color not supported for polygon mesh".to_string(),
                )),
            },
            MeshType::Cloud(_) => match mesh.color() {
                Color::Func(f) => {
                    let vertices = mesh.vertices();
                    let mut gpu_vertices = Vec::new();
                    for (i, v) in vertices.into_iter().enumerate() {
                        gpu_vertices.push(GpuVertex::from(v, &f(v, i), normals.get_normal(i)?));
                    }
                    Ok(gpu_vertices)
                }
                Color::Vertex(colors) => {
                    let vertices = mesh.vertices();
                    vertices_check(colors, vertices)?;
                    let mut gpu_vertices = Vec::new();
                    for ((i, v), c) in vertices.into_iter().enumerate().zip(colors.into_iter()) {
                        gpu_vertices.push(GpuVertex::from(v, c, normals.get_normal(i)?));
                    }
                    Ok(gpu_vertices)
                }
                Color::Face(_) => Err(MeshError::InvalidFaceType(
                    "Face color not supported for cloud mesh".to_string(),
                )),
                Color::Mesh(c) => {
                    let mut gpu_vertices = Vec::new();
                    for (i, v) in mesh.vertices().into_iter().enumerate() {
                        gpu_vertices.push(GpuVertex::from(v, c, normals.get_normal(i)?));
                    }
                    Ok(gpu_vertices)
                }
                Color::Line(colors) => {
                    let mut vertices = Vec::new();
                    for (MeshEdge(a, b), c) in zip(mesh.edges().into_iter(), colors.into_iter()) {
                        let v1 = mesh.get(*a)?;
                        let v2 = mesh.get(*b)?;
                        vertices.push(GpuVertex::from(v1, c, normals.get_normal(*a)?));
                        vertices.push(GpuVertex::from(v2, c, normals.get_normal(*b)?));
                    }
                    Ok(vertices)
                }
            },
            MeshType::Lines => match mesh.color() {
                Color::Face(_) => Err(MeshError::InvalidFaceType(
                    "Face color not supported for cloud mesh".to_string(),
                )),
                Color::Mesh(m) => {
                    let mut vertices = Vec::new();
                    for edge in mesh.edges().iter() {
                        for &i in &[edge.0, edge.1] {
                            let v = mesh.get(i)?;
                            vertices.push(GpuVertex::from(v, m, normals.get_normal(i)?));
                        }
                    }
                    Ok(vertices)
                },

                Color::Func(f) => {
                    let mut vertices = Vec::new();
                    for MeshEdge(a, b) in mesh.edges().into_iter() {
                        let v = mesh.get(*a)?;
                        vertices.push(GpuVertex::from(v, &f(v, *a), normals.get_normal(*a)?));
                        let v = mesh.get(*b)?;
                        vertices.push(GpuVertex::from(v, &f(v, *b), normals.get_normal(*b)?));
                    }
                    Ok(vertices)
                }
                Color::Vertex(colors) => {
                    let mut vertices = Vec::new();
                    for MeshEdge(a, b) in mesh.edges().into_iter() {
                        let v = mesh.get(*a)?;
                        vertices.push(GpuVertex::from(
                            v,
                            &colors.get(*a).ok_or(MeshError::idx_vertex(*a))?.clone(),
                            normals.get_normal(*a)?,
                        ));
                        let v = mesh.get(*b)?;
                        vertices.push(GpuVertex::from(
                            v,
                            &colors.get(*b).ok_or(MeshError::idx_vertex(*b))?.clone(),
                            normals.get_normal(*b)?,
                        ));
                    }

                    Ok(vertices)
                }
                Color::Line(colors) => {
                    let mut vertices = Vec::new();
                    for (MeshEdge(a, b), c) in zip(mesh.edges().into_iter(), colors.into_iter()) {
                        let v1 = mesh.get(*a)?;
                        let v2 = mesh.get(*b)?;
                        vertices.push(GpuVertex::from(v1, c, normals.get_normal(*a)?));
                        vertices.push(GpuVertex::from(v2, c, normals.get_normal(*b)?));
                    }
                    Ok(vertices)
                }
            },
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
        Err(MeshError::InvalidIndex(format!(
            "Vertex color count {} does not match vertex count {}",
            vs.len(),
            vertices.len()
        )))
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
