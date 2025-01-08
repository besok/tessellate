use super::{Mesh, MeshResult};
use crate::mesh::parts::vertex::Vertex;

pub fn distance_to_surface(lhs: &Vertex, rhs: &Mesh) -> MeshResult<f32> {
    Ok(1.0)
}

pub fn distance_between_surfaces(lhs: &Mesh, rhs: &Mesh) -> MeshResult<f32> {
    Ok(1.0)
}
