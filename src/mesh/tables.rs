use crate::mesh::parts::face::Face;
use crate::mesh::parts::Idx;
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashMap;

pub struct MeshTables<'a> {
    mesh: &'a Mesh,
    vert_edges: HashMap<Idx, Vec<Idx>>,
    vert_faces: HashMap<Idx, Vec<Idx>>,
    edge_faces: HashMap<(Idx, Idx), Vec<Idx>>,
}

impl<'a> TryFrom<&'a Mesh> for MeshTables<'a> {
    type Error = MeshError;

    fn try_from(value: &'a Mesh) -> Result<MeshTables<'a>, Self::Error> {
        MeshTables::new(value)
    }
}

impl<'a> MeshTables<'a> {
    fn new(mesh: &'a Mesh) -> MeshResult<Self> {
        let edges = mesh.edges();
        let faces = mesh.faces();

        let mut vertice_edges = HashMap::new();
        let mut edge_faces = HashMap::new();
        let mut vertice_faces = HashMap::new();

        let e_map: HashMap<_, _> = edges
            .iter()
            .enumerate()
            .flat_map(|(i, edge)| edge.indexes().map(|(l, r)| ((l, r), i)))
            .collect();

        for (idx, edge) in edges.iter().enumerate() {
            let (v1, v2) = edge.indexes().ok_or(MeshError::idx_edge(idx, idx))?;
            vertice_edges.entry(v1).or_insert_with(Vec::new).push(idx);
            vertice_edges.entry(v2).or_insert_with(Vec::new).push(idx);
        }

        for (idx, face) in faces.iter().enumerate() {
            match face {
                Face::Triangle(v1, v2, v3) => {
                    edge_faces
                        .entry((*v1, *v2))
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry((*v2, *v3))
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry((*v3, *v1))
                        .or_insert_with(Vec::new)
                        .push(idx);

                    vertice_faces.entry(*v1).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v2).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v3).or_insert_with(Vec::new).push(idx);
                }
                Face::Quad(v1, v2, v3, v4) => {
                    edge_faces
                        .entry((*v1, *v2))
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry((*v2, *v3))
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry((*v3, *v4))
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry((*v4, *v1))
                        .or_insert_with(Vec::new)
                        .push(idx);
                    vertice_faces.entry(*v1).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v2).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v3).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v4).or_insert_with(Vec::new).push(idx);
                }
            }
        }
        Ok(Self {
            mesh,
            vert_edges: vertice_edges,
            edge_faces,
            vert_faces: vertice_faces,
        })
    }

    pub fn vertex_faces_idx(&self, idx: Idx) -> Option<&Vec<Idx>> {
        self.vert_faces.get(&idx)
    }
    pub fn vertex_faces(&self, idx: Idx) -> MeshResult<Vec<&Face>> {
        self.vert_faces
            .get(&idx)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|f_idx| self.get_face(f_idx))
            .collect()
    }

    fn get_face(&self, idx: Idx) -> MeshResult<&Face> {
        self.mesh.faces.get(idx).ok_or(MeshError::idx_face(idx))
    }
    pub fn edge_faces(&self, lhs: Idx, rhs: Idx) -> MeshResult<Vec<&Face>> {
        let faces1 = self.edge_faces.get(&(lhs, rhs)).cloned().unwrap_or_default();
        let faces2 = self.edge_faces.get(&(rhs, lhs)).cloned().unwrap_or_default();
        faces1
            .into_iter()
            .chain(faces2.into_iter())
            .map(|f_idx| self.get_face(f_idx))
            .collect()
    }
}
