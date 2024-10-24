use crate::mesh::parts::edge::{Edge, MeshEdge};
use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshResult};
use std::collections::HashMap;

pub(crate) fn extract_boundary_edges(mesh: &Mesh) -> MeshResult<Vec<Edge>> {
    let mut edges_faces: HashMap<MeshEdge, Vec<&Face>> = HashMap::new();
    for face in mesh.faces() {
        for edge in face.edges() {
            if let Some(faces) = edges_faces.get_mut(&edge) {
                faces.push(face);
            } else {
                edges_faces.insert(edge, vec![face]);
            }
        }
    }

    let mut edges = Vec::new();

    for (edge, faces) in edges_faces.iter_mut() {
        if faces.len() == 1 {
            edges.push(Edge::new(*mesh.get(edge.0)?, *mesh.get(edge.1)?));
        }
    }
    Ok(edges)
}
