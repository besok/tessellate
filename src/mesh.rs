use std::collections::HashSet;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::mesh::normals::VertexNormals;
use crate::mesh::parts::{Face, FaceType, Vertex};
use crate::mesh::tables::MeshTables;
use parts::Edge;
pub mod material;
pub mod normals;
pub mod parts;
pub mod shape;
pub mod tables;

type MeshResult<T> = Result<T, MeshError>;

#[derive(Debug, Clone, PartialEq)]
pub enum MeshError {
    InvalidIndex(String),
    InvalidFaceType(String),
}

impl MeshError {
    fn idx_edge(v1: usize, v2: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the edge: {} - {}", v1, v2))
    }
}

#[derive(Default)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
}

impl Mesh {
    pub fn get_v(&self, idx: usize) -> MeshResult<&Vertex> {
        self.vertices
            .get(idx)
            .ok_or(MeshError::InvalidIndex("Invalid vertex index".to_string()))
    }

    pub fn from_vertices<V, F>(vertices: Vec<V>, faces: Vec<F>) -> Self
    where
        V: Into<Vertex>,

        F: Into<Face>,
    {
        let vertices = vertices.into_iter().map(Into::into).collect();
        let faces: Vec<Face> = faces.into_iter().map(Into::into).collect();
        let edges: Vec<Edge> = dedup(
            faces
                .iter()
                .flat_map(|f| <&Face as Into<Vec<Edge>>>::into(f))
                .collect(),
        );
        Mesh {
            vertices,
            edges,
            faces,
        }
    }

    pub fn try_tables(&self) -> MeshResult<MeshTables> {
        self.try_into()
    }
    pub fn try_normals(&self) -> MeshResult<VertexNormals> {
        self.try_into()
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }
}

pub fn dedup<T: Hash + Eq>(items: Vec<T>) -> Vec<T> {
    items
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::mesh::parts::Idx;
    use glam::Vec3;

    #[test]
    fn test() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let faces: Vec<Face> = vec![(0, 1, 2).into(), (0, 2, 3).into()];
        let mesh = Mesh::from_vertices(vertices, faces);
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.edges.len(), 4);
        assert_eq!(mesh.faces.len(), 2);
    }

    #[test]
    fn test_normals() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let faces: Vec<Face> = vec![(0, 1, 2).into(), (0, 2, 3).into()];
        let mesh = Mesh::from_vertices(vertices, faces);
        let normals = mesh.try_normals().unwrap();

        assert_eq!(normals.get_normal(0), Ok(&Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(1), Ok(&Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(2), Ok(&Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(3), Ok(&Vec3::new(0.0, 0.0, 1.0)));
    }
}
