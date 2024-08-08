use crate::mesh::query::sskdtree::{SSKDNode, SSKDTree};
use crate::mesh::parts::BoundingBox;
use crate::mesh::{MeshError, MeshResult};
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::Axis;

const MAX_REC_DEPTH: usize = 90;
const MIN_POLYGONS: usize = 25;

pub fn try_build_sskd_tree(
    polygons: &Vec<Polygon>,
    depth: Option<usize>,
    min_polygons: Option<usize>,
) -> MeshResult<SSKDTree> {
    let max_depth = depth.unwrap_or(MAX_REC_DEPTH);
    let min_polygons = min_polygons.unwrap_or(MIN_POLYGONS);
    let error = MeshError::Custom("empty tree".to_string());
    let root = build_node(polygons, 0, max_depth, min_polygons).and_then(|v| v.ok_or(error))?;
    Ok(SSKDTree { root })
}

fn build_node(
    polygons: &Vec<Polygon>,
    depth: usize,
    max_depth: usize,
    min_poly: usize,
) -> MeshResult<Option<Box<SSKDNode>>> {
    if polygons.is_empty() {
        Ok(None)
    } else if polygons.len() < min_poly || depth >= max_depth {
        Ok(Some(Box::new(SSKDNode::Leaf {
            bb: BoundingBox::from_polygons(polygons),
            polygons: polygons.clone(),
        })))
    } else {
        let axis = Axis::get(depth);
        let bb = BoundingBox::from_polygons(polygons);
        let split_plane = compute_split_plane(&axis, polygons);
        let (left_polygons, right_polygons) = split_polygons(&polygons, split_plane, &axis)?;

        let left = build_node(&left_polygons, depth + 1, max_depth, min_poly)?;
        let right = build_node(&right_polygons, depth + 1, max_depth, min_poly)?;

        if left.is_none() && right.is_none() {
            Ok(Some(Box::new(SSKDNode::Leaf {
                bb,
                polygons: polygons.clone(),
            })))
        } else {
            Ok(Some(Box::new(SSKDNode::Node {
                bb,
                axis,
                left,
                right,
            })))
        }
    }
}

fn split_bounding_box(
    bounding_box: &BoundingBox,
    split_plane: f32,
    axis: &Axis,
) -> (BoundingBox, BoundingBox) {
    let mut left_bbox = bounding_box.clone();
    let mut right_bbox = bounding_box.clone();
    left_bbox.max_mut().set(axis,split_plane) ;
    right_bbox.min_mut().set(axis,split_plane);
    (left_bbox, right_bbox)
}

fn split_polygons(
    polygons: &Vec<Polygon>,
    split_plane: f32,
    axis: &Axis,
) -> MeshResult<(Vec<Polygon>, Vec<Polygon>)> {
    let mut left_polygons = Vec::new();
    let mut right_polygons = Vec::new();
    for polygon in polygons {
        if polygon.centroid()?.get(axis) <= split_plane {
            left_polygons.push(polygon.clone());
        } else {
            right_polygons.push(polygon.clone());
        }
    }
    Ok((left_polygons, right_polygons))
}

fn compute_split_plane(axis: &Axis, polygons: &Vec<Polygon>) -> f32 {
    // Example heuristic: median of polygon centroids along the chosen axis
    let mut centroids = polygons
        .iter()
        .flat_map(|polygon| polygon.centroid())
        .map(|v| v.get(axis))
        .collect::<Vec<_>>();
    centroids.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    centroids[centroids.len() / 2]
}
