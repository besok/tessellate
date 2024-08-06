use crate::mesh::parts::{Edge, Idx};
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashMap;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;

#[derive(Default)]
pub struct MeshTables {
    v_edges: HashMap<Idx, Vec<Idx>>,
    e_faces: HashMap<Idx, Vec<Idx>>,
}

impl TryFrom<&Mesh> for MeshTables {
    type Error = MeshError;

    fn try_from(value: &Mesh) -> Result<Self, Self::Error> {
        MeshTables::new(&value.vertices, &value.edges, &value.faces)
    }
}

impl MeshTables {
    fn new(vertices: &Vec<Vertex>, edges: &Vec<Edge>, faces: &Vec<Face>) -> MeshResult<Self> {
        let mut v_edges = HashMap::new();
        let mut e_faces = HashMap::new();

        let e_map: HashMap<_, _> = edges
            .iter()
            .enumerate()
            .flat_map(|(i, edge)| edge.indexes().map(|(l, r)| ((l, r), i)))
            .collect();

        for (idx, edge) in edges.iter().enumerate() {
            let (v1, v2) = edge.indexes().ok_or(MeshError::idx_edge(idx, idx))?;
            v_edges.entry(v1).or_insert_with(Vec::new).push(idx);
            v_edges.entry(v2).or_insert_with(Vec::new).push(idx);
        }

        let i = |k: (usize, usize)| -> MeshResult<usize> {
            e_map.get(&k).copied().ok_or(MeshError::idx_edge(k.0, k.1))
        };

        for (idx, face) in faces.iter().enumerate() {
            match face {
                Face::Triangle(v1, v2, v3) => {
                    e_faces
                        .entry(i((*v1, *v2))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    e_faces
                        .entry(i((*v2, *v3))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    e_faces
                        .entry(i((*v3, *v1))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                }
                Face::Quad(v1, v2, v3, v4) => {
                    e_faces
                        .entry(i((*v1, *v2))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    e_faces
                        .entry(i((*v2, *v3))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    e_faces
                        .entry(i((*v3, *v4))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    e_faces
                        .entry(i((*v4, *v1))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                }
            }
        }
        Ok(Self { v_edges, e_faces })
    }
}
