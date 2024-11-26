use crate::mesh::parts::Idx;
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashMap;
use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;

#[derive(Default)]
pub struct MeshTables {
    vertice_edges: HashMap<Idx, Vec<Idx>>,
    vertice_faces: HashMap<Idx, Vec<Idx>>,
    edge_faces: HashMap<Idx, Vec<Idx>>,
}

impl TryFrom<&Mesh> for MeshTables {
    type Error = MeshError;

    fn try_from(value: &Mesh) -> Result<Self, Self::Error> {
        MeshTables::new(&value.vertices, &value.edges, &value.faces)
    }
}

impl MeshTables {
    fn new(_vertices: &Vec<Vertex>, edges: &Vec<MeshEdge>, faces: &Vec<Face>) -> MeshResult<Self> {
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

        let i = |k: (usize, usize)| -> MeshResult<usize> {
            e_map.get(&k).copied().ok_or(MeshError::idx_edge(k.0, k.1))
        };

        for (idx, face) in faces.iter().enumerate() {
            match face {
                Face::Triangle(v1, v2, v3) => {
                    edge_faces
                        .entry(i((*v1, *v2))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry(i((*v2, *v3))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry(i((*v3, *v1))?)
                        .or_insert_with(Vec::new)
                        .push(idx);

                    vertice_faces.entry(*v1).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v2).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v3).or_insert_with(Vec::new).push(idx);

                }
                Face::Quad(v1, v2, v3, v4) => {
                    edge_faces
                        .entry(i((*v1, *v2))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry(i((*v2, *v3))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry(i((*v3, *v4))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    edge_faces
                        .entry(i((*v4, *v1))?)
                        .or_insert_with(Vec::new)
                        .push(idx);
                    vertice_faces.entry(*v1).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v2).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v3).or_insert_with(Vec::new).push(idx);
                    vertice_faces.entry(*v4).or_insert_with(Vec::new).push(idx);

                }
            }
        }
        Ok(Self { vertice_edges, edge_faces, vertice_faces })
    }

    pub fn vertex_faces(&self, idx: Idx) -> Option<&Vec<Idx>> {
        self.vertice_faces.get(&idx)
    }

}
