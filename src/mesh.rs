mod parts;

use crate::mesh::parts::{Array34, BorrowedEdge, Face, Idx, VColor, Vertex, VNormal};
use parts::Edge;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};


#[derive(Default)]
struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    tables: MeshTables,
    attributes: MeshAttributes,
}

impl Mesh {
    pub fn new<V, E, F>(vertices: Vec<V>, edges: Vec<E>, faces: Vec<F>) -> Self
    where
        V: Into<Vertex>,
        E: Into<Edge>,
        F: Into<Face>,
    {
        let vertices = vertices.into_iter().map(|v| v.into()).collect();
        let edges = edges.into_iter().map(|e| e.into()).collect();
        let faces = faces.into_iter().map(|f| f.into()).collect();
        Mesh {
            vertices,
            edges,
            faces,
            ..Default::default()
        }
    }
    pub fn new_by_idx<V, F>(vertices: Vec<V>, edges: Vec<(usize, usize)>, faces: Vec<F>) -> Self
    where
        V: Into<Vertex>,
        F: Into<Array34<usize>>,
    {
        let vertices:Vec<_> = vertices.into_iter().map(|v| v.into()).collect();
        let edges = edges
            .into_iter()
            .map(|(v1, v2)| Edge(vertices[v1], vertices[v2]))
            .collect();
        let faces = faces
            .into_iter()
            .map(|f| match f.into() {
                Array34::Array3(v) => {
                    Face::Triangle(vertices[v[0]], vertices[v[1]], vertices[v[2]])
                }
                Array34::Array4(v) => {
                    Face::Quad(vertices[v[0]], vertices[v[1]], vertices[v[2]], vertices[v[3]])
                }
            })
            .collect();
        Mesh {
            vertices,
            edges,
            faces,
            ..Default::default()
        }
    }
}

struct MeshAttributes {
    v_color: VertexColors,
    v_normals: VertexNormals,
}

impl Default for MeshAttributes {
    fn default() -> Self {
        MeshAttributes {
            v_color: VertexColors::NotPresent,
            v_normals: VertexNormals::NotPresent,
        }
    }
}

enum VertexColors {
    Present(Vec<VColor>),
    NotPresent,
}

enum VertexNormals {
    Present(Vec<VNormal>),
    NotPresent,
}

#[derive(Default)]
struct MeshTables {
    v_edges: HashMap<Idx, Vec<Idx>>,
    e_faces: HashMap<Idx, Vec<Idx>>,
}

impl MeshTables {
    fn new(vertices: &Vec<Vertex>, edges: &Vec<Edge>, faces: &Vec<Face>) -> Self {
        let mut v_edges = HashMap::new();
        let mut e_faces = HashMap::new();

        let vs: HashMap<&Vertex, usize> =
            vertices.iter().enumerate().map(|(i, v)| (v, i)).collect();
        let es: HashMap<BorrowedEdge, usize> = edges
            .iter()
            .enumerate()
            .map(|(i, Edge(v1, v2))| ((v1, v2), i))
            .collect();

        for ((v1, v2), e_idx) in es.iter() {
            v_edges.entry(vs[v1]).or_insert_with(Vec::new).push(*e_idx);
            v_edges.entry(vs[v2]).or_insert_with(Vec::new).push(*e_idx);
        }

        for (face_idx, face) in faces.iter().enumerate() {
            match face {
                Face::Triangle(v1, v2, v3) => {
                    let e1 = es.get(&(&v1, &v2)).unwrap();
                    let e2 = es.get(&(v2, v3)).unwrap();
                    let e3 = es.get(&(v3, v1)).unwrap();
                    e_faces.entry(*e1).or_insert_with(Vec::new).push(face_idx);
                    e_faces.entry(*e2).or_insert_with(Vec::new).push(face_idx);
                    e_faces.entry(*e3).or_insert_with(Vec::new).push(face_idx);
                }
                Face::Quad(v1, v2, v3, v4) => {
                    let e1 = es.get(&(v1, v2)).unwrap();
                    let e2 = es.get(&(v2, v3)).unwrap();
                    let e3 = es.get(&(v3, v4)).unwrap();
                    let e4 = es.get(&(v4, v1)).unwrap();
                    e_faces.entry(*e1).or_insert_with(Vec::new).push(face_idx);
                    e_faces.entry(*e2).or_insert_with(Vec::new).push(face_idx);
                    e_faces.entry(*e3).or_insert_with(Vec::new).push(face_idx);
                    e_faces.entry(*e4).or_insert_with(Vec::new).push(face_idx);
                }
            }
        }

        Self { v_edges, e_faces }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let faces = vec![[0, 1, 2], [0, 2, 3]];
        let mesh = Mesh::new_by_idx(vertices, edges, faces);
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.edges.len(), 4);
        assert_eq!(mesh.faces.len(), 2);
    }
}
