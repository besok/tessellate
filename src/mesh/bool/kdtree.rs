use crate::mesh::bool::bsp::BSPTree;
use crate::mesh::bool::kdtree::query::KDTreeIter;
use crate::mesh::material::Color;
use crate::mesh::parts::{Face, Vertex};
use crate::mesh::{Mesh, MeshError, MeshResult};
use glam::Vec3;

pub mod build;
pub mod query;

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
    pub fn iter_preorder(&self) -> KDTreeIter {
        KDTreeIter::new(&self.root)
    }

    pub fn iter_inorder(&self) -> KDTreeIter {
        let mut iter = KDTreeIter::empty();
        iter.push_left(&self.root);
        iter
    }

    pub fn iter_postorder(&self) -> KDTreeIter {
        let mut iter = KDTreeIter::empty();
        iter.push_left_postorder(&self.root);
        iter
    }
}

impl TryFrom<&Mesh> for KDTree {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> MeshResult<Self> {
        KDTree::try_from_mesh(mesh, None)
    }
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::gpu::camera::position::CameraPosition;
    use crate::gpu::visualize;
    use crate::mesh::bool::bsp::{BSPNode, BSPTree};
    use crate::mesh::bool::kdtree::{KDNode, KDTree};
    use crate::mesh::shape::cone::Cone;
    use crate::mesh::shape::cuboid::cube::Cube;
    use crate::mesh::shape::sphere::Sphere;
    use crate::mesh::HasMesh;
    use crate::turn_on_test_logs;

    #[test]
    fn smoke_test() {
        turn_on_test_logs();
        let fig = Cone::default();
        let mesh = fig.mesh();
        let kdtree: KDTree = mesh.try_into().unwrap();
        fn print_tree(node: &Box<KDNode>, depth: usize) {
            let node = node.as_ref();
            match node {
                KDNode::Leaf { point, .. } => {
                    println!("{}Leaf: {:?}", " ".repeat(depth), point);
                }
                KDNode::Node {
                    point,
                    left,
                    right,
                    axis,
                } => {
                    println!("{}Node: {:?}", " ".repeat(depth), point);
                    if let Some(left) = left {
                        print_tree(left, depth + 1);
                    }

                    if let Some(right) = right {
                        print_tree(right, depth + 1);
                    }
                }
            }
        }
        print_tree(kdtree.root(), 0);

    }


}
