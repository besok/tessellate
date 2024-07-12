use std::hash::{Hash, Hasher};

use parts::Edge;

use crate::mesh::normals::VertexNormals;
use crate::mesh::parts::{Face, Vertex};
use crate::mesh::tables::MeshTables;

type MeshResult<T> = Result<T, MeshError>;

#[derive(Debug, Clone, PartialEq)]
enum MeshError {
    InvalidIndex(String),
}

impl MeshError {
    fn idx_edge(v1: usize, v2: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the edge: {} - {}", v1, v2))
    }
}

pub mod normals;
pub mod parts;
pub mod tables;
mod primitives;

pub type VColor = [f32; 4];

#[derive(Default)]
struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    v_colors: VertexColors,
}

impl Mesh {

    pub fn get_v(&self, idx: usize) -> MeshResult<&Vertex> {
        self.vertices.get(idx).ok_or(MeshError::InvalidIndex("Invalid vertex index".to_string()))
    }

    pub fn from_vertices<V, E, F>(
        vertices: Vec<V>,
        edges: Vec<E>,
        faces: Vec<F>,
        v_colors: VertexColors,
    ) -> Self
    where
        V: Into<Vertex>,
        E: Into<Edge>,
        F: Into<Face>,
    {
        let vertices = vertices.into_iter().map(Into::into).collect();
        let edges = edges.into_iter().map(Into::into).collect();
        let faces: Vec<Face> = faces.into_iter().map(Into::into).collect();
        Mesh {
            vertices,
            edges,
            faces,
            v_colors,
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

enum VertexColors {
    Present(Vec<VColor>),
    NotPresent,
}

impl Default for VertexColors {
    fn default() -> Self {
        VertexColors::NotPresent
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::mesh::parts::Idx;

    #[test]
    fn test() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let edges: Vec<(Idx, Idx)> = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let faces: Vec<Array34> = vec![[0, 1, 2].into(), [0, 2, 3].into()];
        let mesh = Mesh::from_vertices(vertices, edges, faces, Default::default());
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.edges.len(), 4);
        assert_eq!(mesh.faces.len(), 2);
    }

    #[test]
    fn test_normals() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let edges: Vec<(Idx, Idx)> = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let faces: Vec<Array34> = vec![[0, 1, 2].into(), [0, 2, 3].into()];
        let mesh = Mesh::from_vertices(vertices, edges, faces, Default::default());
        let normals = mesh.try_normals().unwrap();

        assert_eq!(normals.get_normal(0), Ok(&Vector3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(1), Ok(&Vector3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(2), Ok(&Vector3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(3), Ok(&Vector3::new(0.0, 0.0, 1.0)));
    }
}
