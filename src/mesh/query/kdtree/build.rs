use crate::mesh::query::kdtree::{KDNode, KDTree};
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::{MeshError, MeshResult};
use std::cmp::Ordering;
use crate::mesh::parts::vertex::Vertex;

const MAX_REC_DEPTH: usize = 90;

pub fn try_build_kd_tree(polygons: &Vec<Polygon>, depth: Option<usize>) -> MeshResult<KDTree> {
    let max_depth = depth.unwrap_or(MAX_REC_DEPTH);
    let error = MeshError::Custom("empty tree".to_string());
    let root = build_node(polygons, 0, max_depth).and_then(|v| v.ok_or(error))?;
    Ok(KDTree { root })
}

fn build_node(
    polygons: &Vec<Polygon>,
    depth: usize,
    max_depth: usize,
) -> MeshResult<Option<Box<KDNode>>> {
    let axis = depth % 3;
    let by_axis = |(_, p1): &(_, Vertex), (_, p2): &(_, Vertex)| sort_by_axis(&p1, &p2, axis);

    if polygons.is_empty() || depth >= max_depth {
        Ok(None)
    } else if polygons.len() == 1 {
        Ok(Some(Box::new(KDNode::Leaf { point: polygons[0].centroid()?, axis })))
    } else {
        let mut points = polygons
            .iter()
            .enumerate()
            .map(|(idx, p)| p.centroid().map(|v| (idx, v)))
            .collect::<Result<Vec<_>, _>>()?;
        points.sort_by(by_axis);

        let median = points.len() / 2;
        let (idx, point) = points[median].clone();
        let (left, right) = polygons.split_at(idx);

        let left = build_node(&left.to_vec(), depth + 1, max_depth)?;
        let right = build_node(&right.to_vec(), depth + 1, max_depth)?;

        if left.is_none() && right.is_none() {
            Ok(Some(Box::new(KDNode::Leaf { point, axis })))
        } else {
            Ok(Some(Box::new(KDNode::Node {
                point,
                left,
                right,
                axis,
            })))
        }
    }
}

fn sort_by_axis(v1: &Vertex, v2: &Vertex, axis: usize) -> Ordering {
    v1.flatten()[axis]
        .partial_cmp(&v2.flatten()[axis])
        .unwrap_or(Ordering::Equal)
}
