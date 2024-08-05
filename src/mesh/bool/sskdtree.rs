mod build;
mod query;

use crate::mesh::material::Color;
use crate::mesh::parts::{Axis, BoundingBox, FaceType, Polygon};
use crate::mesh::{HasMesh, Mesh, MeshError, MeshResult};

/// KDTree Spatial Subdivision Mesh Intersection
///
/// This module contains the implementation of the KDTree spatial subdivision
/// structure. The KDTree is a binary tree that recursively splits the space
/// along the x, y, and z axes. This allows for faster intersection testing
/// by reducing the number of triangles that need to be tested for intersections
/// with a ray.
///
/// The KDTree is implemented as a binary tree where each node has a bounding
/// box and an axis along which the space is split. The tree is built by
/// recursively splitting the space into two halves along the axis with the
/// largest variance. This is done until a maximum depth is reached or the number
/// of triangles in the leaf node is below a threshold. When a ray is tested for
/// intersection with the KDTree, the ray is intersected with the bounding box
/// of the root node. If the ray intersects the bounding box, the ray is tested
/// against the children nodes. If the ray intersects a leaf node, the ray is
/// tested against the triangles in the leaf node.
///
/// The KDTree is used to accelerate the mesh intersection tests in the mesh
/// structure. The KDTree is built from the triangles of the mesh and stored
/// in the mesh structure. When a ray is tested for intersection with the mesh,
/// the ray is intersected with the KDTree instead of the triangles directly.
/// This reduces the number of intersection tests that need to be performed
/// and improves the performance of the intersection tests.
///
/// The KDTree structure is implemented as a binary tree where each node has
/// a bounding box and an axis along which the space is split. The KDTree
/// structure is implemented as a binary tree where each node has a bounding
/// box and an axis along which the space is split. The tree is built by
/// recursively splitting the space into two halves along the axis with the
/// largest variance. This is done until a maximum depth is reached or the number
/// of triangles in the leaf node is below a threshold.
///
/// The KDTree structure is implemented as a binary tree where each node has
/// a bounding box and an axis along which the space is split. The tree is
/// built by recursively splitting the space into two halves along the axis
/// with the largest variance. This is done until a maximum depth is reached
/// or the number of triangles in the leaf node is below a threshold.

#[derive(Debug, Clone, PartialEq)]
pub enum SSKDNode {
    Leaf {
        bb: BoundingBox,
        polygons: Vec<Polygon>,
    },
    Node {
        bb: BoundingBox,
        axis: Axis,
        left: Option<Box<SSKDNode>>,
        right: Option<Box<SSKDNode>>,
    },
}

impl SSKDNode {
    pub fn polygons(&self) -> Vec<Polygon> {
        match self {
            SSKDNode::Leaf { polygons, .. } => polygons.clone(),
            SSKDNode::Node { .. } => vec![],
        }
    }
    pub fn bbox(&self) -> BoundingBox {
        match self {
            SSKDNode::Leaf { bb, .. } => *bb,
            SSKDNode::Node { bb, .. } => *bb,
        }
    }
}

pub struct SSKDTree {
    root: Box<SSKDNode>,
}

impl SSKDTree {
    pub fn try_from_mesh(
        mesh: &Mesh,
        depth: Option<usize>,
        min_polygons: Option<usize>,
    ) -> MeshResult<Self> {
        build::try_build_sskd_tree(&mesh.try_polygons()?, depth, min_polygons)
    }

    pub fn root(&self) -> &Box<SSKDNode> {
        &self.root
    }

    pub fn iter(&self) -> query::InOrderIter {
        query::InOrderIter::new(&self.root)
    }

    pub fn bb_to_mesh(&self) -> Vec<Mesh> {
        self.iter()
            .map(|node| node.bbox())
            .map(|bb| bb.to_rect_cuboid(FaceType::Triangle, Color::default()))
            .map(|cuboid| cuboid.mesh().clone())
            .collect()
    }

    pub fn to_mesh(&self, color: Color) -> Mesh {
        let polygons = self.iter().flat_map(|node| node.polygons()).collect();
        Mesh::from_polygons(polygons, color)
    }
}

impl TryFrom<&Mesh> for SSKDTree {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> MeshResult<Self> {
        SSKDTree::try_from_mesh(mesh, None, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::bool::kdtree::KDTree;
    use crate::mesh::bool::sskdtree::{SSKDNode, SSKDTree};
    use crate::mesh::parts::Vertex;
    use crate::mesh::shape::cone::Cone;
    use crate::mesh::HasMesh;

    #[test]
    fn smoke_test() {
        let fig = Cone::default();
        let mesh = fig.mesh();
        let kdtree: SSKDTree = mesh.try_into().unwrap();

        for node in kdtree.iter() {
            match node {
                SSKDNode::Leaf { polygons, .. } => {
                    for poly in polygons {
                        println!("Polygon: {:?}", poly);
                    }
                }
                SSKDNode::Node { .. } => {}
            }
        }
    }
}
