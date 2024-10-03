mod analyzer;

use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::query::MeshQuery;
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashSet;

pub enum BoolType {
    Union, // A ∪ B
    Intersection, // A ∩ B
    Difference, // A - B
    SymmetricDifference, // A ⊕ B
}
/// Perform a boolean operation on two meshes
/// The implementation from this function is based on the paper:
/// Exact, Robust, and Efficient Boolean Operations (Cork Algorithm)
pub fn perform_bool(
    mesh_a: &Mesh,
    mesh_b: &Mesh,
    op: BoolType,
    color: Option<Color>,
) -> MeshResult<Mesh> {
    Err(MeshError::Custom("not implemented".to_string()))
}

