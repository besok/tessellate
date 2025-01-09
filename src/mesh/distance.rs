use super::{Mesh, MeshResult};
use crate::mesh::parts::vertex::Vertex;

/// Calculate the distance from a vertex to the surface of a mesh
///
/// Note1: The mesh must be watertight and closed.
/// Note2: It uses existing KDTree implementation with vertices so the precision is limited to the vertices.
/// # Arguments
///
/// * `lhs` - A reference to the `Vertex` from which the distance is measured
/// * `rhs` - A reference to the `Mesh` to which the distance is measured
///
/// # Returns
///
/// * `MeshResult<f32>` - A result containing the distance or an error

pub fn distance_to_surface(lhs: &Vertex, rhs: &Mesh) -> MeshResult<f32> {
    if !rhs.properties().is_watertight() {
        return Err("Mesh must be watertight and closed".into());
    }

    rhs.query()
        .try_kd_tree(None)?
        .nearest_neighbors(lhs, None)
        .min()
        .map(|n| n.distance)
        .ok_or("No nearest neighbor found".into())
}
/// Calculate the distance between the surfaces of two meshes
///
/// Note1: Both meshes must be watertight and closed.
/// Note2: It uses existing KDTree implementation with vertices so the precision is limited to the vertices.
///
/// # Arguments
///
/// * `lhs` - A reference to the first `Mesh`
/// * `rhs` - A reference to the second `Mesh`
///
/// # Returns
///
/// * `MeshResult<f32>` - A result containing the distance or an error
pub fn distance_between_surfaces(lhs: &Mesh, rhs: &Mesh) -> MeshResult<f32> {
    if !rhs.properties().is_watertight() || !lhs.properties().is_watertight() {
        return Err("Mesh must be watertight and closed".into());
    }

    let rhs_tree = rhs.query().try_kd_tree(None)?;

    lhs.vertices()
        .iter()
        .map(|v| rhs_tree.nearest_neighbors(v, None).min())
        .flatten()
        .min()
        .map(|n| n.distance)
        .ok_or("No nearest neighbor found".into())
}
