use crate::mesh::parts::edge::{Edge, MeshEdge};
use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshResult};
use glam::Vec3;
use std::collections::HashMap;
use std::f32::consts::PI;

fn edges_map(mesh: &Mesh) -> HashMap<MeshEdge, Vec<&Face>> {
    let mut edges_faces: HashMap<MeshEdge, Vec<&Face>> = HashMap::new();
    for face in mesh.faces() {
        for edge in face.edges() {
            edges_faces.entry(edge).or_insert_with(Vec::new).push(face);
        }
    }
    edges_faces
}

fn extract_edges<F>(mesh: &Mesh, factor: F) -> MeshResult<Vec<Edge>>
where
    F: Fn(usize) -> bool,
{
    let mut edges = Vec::new();

    for (edge, faces) in edges_map(mesh).iter_mut() {
        if factor(faces.len()) {
            edges.push(Edge::new(*mesh.get(edge.0)?, *mesh.get(edge.1)?));
        }
    }
    Ok(edges)
}

pub(crate) fn extract_boundary_edges(mesh: &Mesh) -> MeshResult<Vec<Edge>> {
    extract_edges(mesh, |len| len == 1)
}
pub(crate) fn extract_manifold_edges(mesh: &Mesh) -> MeshResult<Vec<Edge>> {
    extract_edges(mesh, |len| len == 2)
}
pub(crate) fn extract_non_manifold_edges(mesh: &Mesh) -> MeshResult<Vec<Edge>> {
    extract_edges(mesh, |len| len > 2)
}
pub(crate) fn extract_feature_edges(mesh: &Mesh, feature_angle: f32) -> MeshResult<Vec<Edge>> {
    let mut edges = Vec::new();
    let edges_faces = edges_map(mesh);
    let normals = mesh.try_normals()?;

    for (edge, faces) in edges_faces.iter() {
        if faces.len() == 2 {
            let lhs_n = normals.get_face_normal(faces[0])?;
            let rhs_n = normals.get_face_normal(faces[1])?;
            if dihedral_angle(lhs_n, rhs_n) > feature_angle * (PI / 180.0) {
                edges.push(Edge::new(*mesh.get(edge.0)?, *mesh.get(edge.1)?));
            }
        }
    }

    Ok(edges)
}

fn dihedral_angle(lhs_normal: &Vec3, rhs_normal: &Vec3) -> f32 {
    lhs_normal.dot(*rhs_normal).acos()
}
