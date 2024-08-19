use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::parts::bbox::BoundingBox;
use crate::mesh::query::octree::{OctNode, Octree};
use crate::mesh::{MeshError, MeshResult};

const MAX_REC_DEPTH: usize = 20;
const MAX_POLY: usize = 50;

pub fn try_build_octree(polygons: &Vec<Polygon>, depth: Option<usize>) -> MeshResult<Octree> {
    let max_depth = depth.unwrap_or(MAX_REC_DEPTH);
    let root = build_node(polygons, 0, max_depth)?;
    Ok(Octree { root })
}

fn build_node(polygons: &Vec<Polygon>, depth: usize, max_depth: usize) -> MeshResult<Box<OctNode>> {
    if depth >= max_depth || polygons.len() <= MAX_POLY {
        Ok(Box::new(OctNode::Leaf {
            bb: BoundingBox::from_polygons(polygons),
            polygons: polygons.clone(),
            depth,
        }))
    } else {
        let bb = BoundingBox::from_polygons(polygons);
        let mut children_polygons = vec![Vec::new(); 8];

        let child_bbs = subdivide(&bb);

        for polygon in polygons {
            for (i, child_bb) in child_bbs.iter().enumerate() {
                if child_bb.intersects_polygon(polygon) {
                    children_polygons[i].push(polygon.clone());
                }
            }
        }

        let mut children = Vec::with_capacity(8);
        for child_polygons in children_polygons {
            children.push(build_node(&child_polygons, depth + 1, max_depth)?);
        }

        let children: [Box<OctNode>; 8] = children
            .try_into()
            .map_err(|_| MeshError::Custom("Failed to convert Vec to array".to_string()))?;

        Ok(Box::new(OctNode::Node { bb, children }))
    }
}

fn subdivide(bb: &BoundingBox) -> [BoundingBox; 8] {
    let center = bb.center();
    let min = bb.min();
    let max = bb.max();
    [
        BoundingBox::new(min.clone(), center),
        BoundingBox::new(
            Vertex::new(center.x, min.y, min.z),
            Vertex::new(max.x, center.y, center.z),
        ),
        BoundingBox::new(
            Vertex::new(min.x, center.y, min.z),
            Vertex::new(center.x, max.y, center.z),
        ),
        BoundingBox::new(
            Vertex::new(center.x, center.y, min.z),
            Vertex::new(max.x, max.y, center.z),
        ),
        BoundingBox::new(
            Vertex::new(min.x, min.y, center.z),
            Vertex::new(center.x, center.y, max.z),
        ),
        BoundingBox::new(
            Vertex::new(center.x, min.y, center.z),
            Vertex::new(max.x, center.y, max.z),
        ),
        BoundingBox::new(
            Vertex::new(min.x, center.y, center.z),
            Vertex::new(center.x, max.y, max.z),
        ),
        BoundingBox::new(center, max.clone()),
    ]
}
