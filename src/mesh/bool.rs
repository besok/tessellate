use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::query::MeshQuery;
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashSet;

pub enum BoolType {
    Union,
    Intersection,
    Difference,
}
/// Perform a boolean operation on two meshes
/// The implementation from this function is based on the paper:
/// "Interactive and Robust Mesh Booleans"
pub fn perform_bool(
    mesh_a: &Mesh,
    mesh_b: &Mesh,
    op: BoolType,
    depth: Option<usize>,
    color: Option<Color>,
) -> MeshResult<Mesh> {
    let props_a = mesh_a.props();
    let props_b = mesh_b.props();
    if !props_a.is_volume()? || !props_b.is_volume()? {
        Err(MeshError::WrongMesh("Meshes must be volumes".to_string()))
    } else {
        let a_tree = MeshQuery::new(mesh_a).try_octree(depth)?;
        let b_tree = MeshQuery::new(mesh_b).try_octree(depth)?;
        let mut inter_polys = Vec::new();
        let mut other_polys_a = HashSet::new();
        let mut other_polys_b = HashSet::new();
        for a_leaf in a_tree.iter_leafs() {
            for b_leaf in b_tree.iter_leafs() {
                let a_polygons = a_leaf.polygons();
                let b_polygons = b_leaf.polygons();
                if a_leaf.is_overlapping(b_leaf) {
                    for a_poly in a_polygons.iter() {
                        for b_poly in b_polygons.iter() {
                            if a_poly.intersects_precise(b_poly)? {
                                inter_polys.push((a_poly.clone(), b_poly.clone()));
                            } else {
                                other_polys_a.insert(a_poly.clone());
                                other_polys_b.insert(b_poly.clone());
                            }
                        }
                    }
                }else {
                    other_polys_a.extend(a_polygons.iter().cloned());
                    other_polys_b.extend(b_polygons.iter().cloned());
                }
            }
        }

        Err(MeshError::Custom("Boolean operations are not implemented yet".to_string()))
    }
}

fn compute_multiplier(coords: Vertex) -> f64 {
    const R: f32 = 11259470696.0; // avg_max_coord (167.78) * old_multiplier (67108864.0)

    let abs_max_coord = coords
        .flatten()
        .iter()
        .map(|&c| c.abs())
        .fold(f32::NEG_INFINITY, f32::max);

    let div = R / abs_max_coord;

    // closest power of 2
    let e = div.log2().round() as i32;
    let multiplier = if e >= 0 {
        (1 << e) as f64
    } else {
        1.0 / (1 << (-e)) as f64
    };

    multiplier.max(1.0)
}
