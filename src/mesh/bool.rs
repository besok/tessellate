use crate::mesh::{Mesh, MeshResult};
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::query::MeshQuery;

pub enum BoolType {
    Union,
    Intersection,
    Difference,
}

impl BoolType {
    pub fn is_union(&self) -> bool {
        match self {
            BoolType::Union => true,
            _ => false,
        }
    }
}

pub fn perform_bool(lhs:&Mesh, rhs:&Mesh, op:BoolType, depth:Option<usize>) -> MeshResult<Mesh> {
    let lhs_tree = MeshQuery(lhs).try_octree(depth)?;
    let rhs_tree = MeshQuery(rhs).try_octree(depth)?;

    let mut inter_polygons = Vec::new();
    let mut non_inter_polygons = Vec::new();

    for lhs_leaf in lhs_tree.iter_leafs() {
        for rhs_leaf in rhs_tree.iter_leafs() {
            if lhs_leaf.is_overlapping(rhs_leaf) {
                let lhs_polygons = lhs_leaf.polygons();
                let rhs_polygons = rhs_leaf.polygons();
                for lhs_p in lhs_polygons.iter() {
                    for rhs_p in rhs_polygons.iter() {
                        if lhs_p.intersects(rhs_p)?{
                            let split_polygons:Vec<Polygon> = split_polygons(lhs_p,rhs_p)?;
                            inter_polygons.extend(split_polygons)
                        }
                    }

                }
            }
        }
    }

    let mut final_polygons = Vec::new();

    for p in inter_polygons.iter() {
        match op {
            BoolType::Union => {
                if is_outside(p, rhs) {
                    final_polygons.push(p.clone());
                }
            }
            BoolType::Intersection => {
                if is_inside(p, rhs) && is_inside(p, lhs) {
                    final_polygons.push(p.clone());
                }
            }
            BoolType::Difference => {
                if is_outside(p, rhs) {
                    final_polygons.push(p.clone());
                }
            }
        }
    }

    Ok(Mesh::default())
}

fn split_polygons(lhs: &Polygon, rhs: &Polygon) -> MeshResult<Vec<Polygon>> {
    Ok(Vec::new())
}

fn is_outside(p: &Polygon, mesh: &Mesh) -> bool {
    false
}
fn is_inside(p: &Polygon, mesh: &Mesh) -> bool {
    false
}