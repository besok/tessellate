use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::BoundingBox;
use crate::mesh::query::MeshQuery;
use crate::mesh::{Mesh, MeshError, MeshResult};

pub mod build;
pub mod query;

#[derive(Debug)]
pub enum OctNode {
    Leaf {
        bb: BoundingBox,
        polygons: Vec<Polygon>,
        depth: usize,
    },
    Node {
        bb: BoundingBox,
        children: [Box<OctNode>; 8],
    },
}

impl OctNode {
    pub fn polygons(&self) -> Vec<Polygon> {
        match self {
            OctNode::Leaf { ref polygons, .. } => polygons.clone(),
            OctNode::Node { .. } => vec![],
        }
    }

    pub fn bb(&self) -> &BoundingBox {
        match self {
            OctNode::Leaf { ref bb, .. } => bb,
            OctNode::Node { ref bb, .. } => bb,
        }
    }

    /// Find all polygons in the octree that intersect with the given bounding box.
    /// This method is used to find all polygons in the octree
    /// that intersect with the given bounding box.
    ///
    /// # Arguments
    /// bb - The bounding box to check for intersections.
    pub fn find_polygons(&self, bb: &BoundingBox) -> Vec<Polygon> {
        match self {
            OctNode::Leaf { ref polygons, .. } => polygons
                .iter()
                .filter(|p| bb.intersects_polygon(p))
                .cloned()
                .collect(),
            OctNode::Node { ref children, .. } => {
                let mut result = Vec::new();
                for child in children.iter() {
                    if child.bb().intersects(bb) {
                        result.extend(child.find_polygons(bb));
                    }
                }
                result
            }
        }
    }

    pub fn is_overlapping(&self, node: &OctNode) -> bool {
        self.bb().intersects(node.bb())
    }
}

/// An octree is a tree data structure in which each internal node has exactly eight children.
/// Octrees are most often used to partition a three-dimensional space by recursively
/// subdividing it into eight octants.
/// Octrees are the three-dimensional analog of quadtrees.
pub struct Octree {
    root: Box<OctNode>,
}

impl Octree {
    pub fn try_from_mesh(mesh: &Mesh, depth: Option<usize>) -> MeshResult<Self> {
        build::try_build_octree(&mesh.try_polygons()?, depth)
    }

    pub fn find_polygons(&self, bb: &BoundingBox) -> Vec<Polygon> {
        self.root.find_polygons(bb)
    }

    pub fn iter(&self) -> query::OctreeIterator {
        query::OctreeIterator::new(&self.root, false)
    }

    pub fn iter_leafs(&self) -> query::OctreeIterator {
        query::OctreeIterator::new(&self.root, true)
    }
}

impl<'a> TryFrom<MeshQuery<'a>> for Octree {
    type Error = MeshError;
    fn try_from(q: MeshQuery<'a>) -> MeshResult<Self> {
        q.try_octree(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::material::Color;
    use crate::mesh::parts::polygon::Polygon;
    use crate::mesh::parts::vertex::Vertex;
    use crate::mesh::query::octree::{OctNode, Octree};
    use crate::mesh::Mesh;

    #[test]
    fn smoke() {
        let polygons = vec![
            Polygon::new(vec![
                &Vertex::new(0.0, 0.0, 0.0),
                &Vertex::new(1.0, 0.0, 0.0),
                &Vertex::new(0.0, 1.0, 0.0),
            ]),
            Polygon::new(vec![
                &Vertex::new(1.0, 1.0, 0.0),
                &Vertex::new(2.0, 1.0, 0.0),
                &Vertex::new(1.0, 2.0, 0.0),
            ]),
        ];
        let mesh = Mesh::from_polygons(polygons, Color::default());
        let octree = Octree::try_from_mesh(&mesh, Some(3)).expect("Failed to build octree");

        match *octree.root {
            OctNode::Leaf {
                ref bb,
                ref polygons,
                depth,
            } => {
                assert_eq!(depth, 0);
                assert_eq!(polygons.len(), 2);
                assert!(bb.contains(&Vertex::new(0.0, 0.0, 0.0)));
                assert!(bb.contains(&Vertex::new(2.0, 2.0, 0.0)));
            }
            OctNode::Node { .. } => panic!("Expected root to be a leaf node"),
        }
    }

    #[test]
    fn test_find_polygons() {
        let polygons = vec![
            Polygon::new(vec![
                &Vertex::new(0.0, 0.0, 0.0),
                &Vertex::new(1.0, 0.0, 0.0),
                &Vertex::new(0.0, 1.0, 0.0),
            ]),
            Polygon::new(vec![
                &Vertex::new(1.0, 1.0, 0.0),
                &Vertex::new(2.0, 1.0, 0.0),
                &Vertex::new(1.0, 2.0, 0.0),
            ]),
        ];
        let mesh = Mesh::from_polygons(polygons, Color::default());
        let octree = Octree::try_from_mesh(&mesh, Some(3)).expect("Failed to build octree");

        let bb = octree.root.bb().clone();
        let result = octree.find_polygons(&bb);
        assert_eq!(result.len(), 2);
    }
}
