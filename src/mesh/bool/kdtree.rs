use crate::mesh::bool::kdtree::query::{InOrderIter, KDTreeNearestNeighborIter};
use crate::mesh::parts::Vertex;
use crate::mesh::{Mesh, MeshError, MeshResult};

pub mod build;
pub mod query;

/// A KDTree is a binary tree that splits the space into two parts at each level.
/// The split is done along one of the axes of the space.
/// The axis is chosen in a round-robin fashion.
/// The tree is built from a list of points.
/// The tree is used to find the nearest neighbors of a given point.
pub struct KDTree {
    root: Box<KDNode>,
}

impl KDTree {
    pub fn try_from_mesh(mesh: &Mesh, depth: Option<usize>) -> MeshResult<Self> {
        build::try_build_kd_tree(&mesh.try_polygons()?, depth)
    }

    pub fn root(&self) -> &Box<KDNode> {
        &self.root
    }
}

impl KDTree {
    /// Find the nearest neighbors of a given point.
    /// The result is an iterator that returns the neighbors in order of increasing distance.
    /// The iterator returns a Neighbour object that contains the node
    /// and the distance of the node from the target.
    ///
    /// # Arguments
    /// * `target` - The target point.
    /// * `max_dist` - The maximum distance of the neighbors from the target.
    /// If None, all the neighbors are returned.
    ///
    /// # Example
    /// ```
    /// use tessellate::mesh::bool::kdtree::KDTree;
    /// use tessellate::mesh::bool::kdtree::query::Neighbour;
    /// use tessellate::mesh::HasMesh;
    /// use tessellate::mesh::parts::Vertex;
    /// use tessellate::mesh::shape::cone::Cone;
    ///
    /// let fig = Cone::default();
    /// let mesh = fig.mesh();
    /// let kdtree: KDTree = mesh.try_into().unwrap();
    ///
    /// for Neighbour { node, distance } in kdtree.nearest_neighbors(&Vertex::default(), None) {
    ///     println!("{:?} - {:?}", node.point(), distance);
    /// }
    /// ```
    pub fn nearest_neighbors<'a>(
        &'a self,
        target: &'a Vertex,
        max_dist: Option<f32>,
    ) -> KDTreeNearestNeighborIter<'a> {
        KDTreeNearestNeighborIter::new(&self.root, target,max_dist)
    }

    pub fn iter(&self) -> InOrderIter {
        InOrderIter::new(&self.root)
    }
}

impl TryFrom<&Mesh> for KDTree {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> MeshResult<Self> {
        KDTree::try_from_mesh(mesh, None)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KDNode {
    Leaf {
        point: Vertex,
        axis: usize,
    },
    Node {
        point: Vertex,
        left: Option<Box<KDNode>>,
        right: Option<Box<KDNode>>,
        axis: usize,
    },
}

impl KDNode {
    pub fn point(&self) -> &Vertex {
        match self {
            KDNode::Leaf { point, .. } => point,
            KDNode::Node { point, .. } => point,
        }
    }

    pub fn axis(&self) -> usize {
        match self {
            KDNode::Leaf { axis, .. } => *axis,
            KDNode::Node { axis, .. } => *axis,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::bool::kdtree::KDTree;
    use crate::mesh::parts::Vertex;
    use crate::mesh::shape::cone::Cone;
    use crate::mesh::HasMesh;

    #[test]
    fn smoke_test() {
        let fig = Cone::default();
        let mesh = fig.mesh();
        let kdtree: KDTree = mesh.try_into().unwrap();

        let full_len = kdtree.nearest_neighbors(&Vertex::default(), None).count();
        let part_len = kdtree.nearest_neighbors(&Vertex::default(), Some(0.7)).count();

        assert_eq!(full_len, 62);
        assert_eq!(part_len, 14);

    }
}
