use std::ops::Deref;
use crate::mesh::bool::bsp::build::try_build_bsp_tree;
use crate::mesh::parts::{Polygon, Vertex};
use crate::mesh::{Mesh, MeshError};
use glam::Vec3;
use rand::Rng;

pub mod build;
pub mod query;

pub struct BSPTree {
    root: Box<BSPNode>,
}

impl BSPTree {
    pub fn root(&self) -> &BSPNode {
        &self.root
    }

    /// Iterates over the tree in preorder
    /// Preorder: root, left, right
    pub fn iter_preorder(&self) -> query::BspPreorderIterator {
        query::BspPreorderIterator::new(&self.root)
    }

    /// Iterates over the tree in inorder
    /// Inorder: left, root, right
    pub fn iter_inorder(&self) -> query::InOrderIterator {
        query::InOrderIterator::new(&self.root)
    }

    /// Iterates over the tree in postorder
    /// Postorder: left, right, root
    pub fn iter_postorder(&self) -> query::PostOrderIterator {
        query::PostOrderIterator::new(&self.root)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BSPNode {
    Leaf {
        polygons: Vec<Polygon>,
    },
    Node {
        front: Box<BSPNode>,
        back: Box<BSPNode>,
        polygons: Vec<Polygon>,
        plane: Plane,
    },
}

impl BSPNode {
    pub fn polygons(&self) -> &Vec<Polygon> {
        match self {
            BSPNode::Leaf { polygons } => polygons,
            BSPNode::Node { polygons, .. } => polygons,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
struct Plane {
    normal: Vec3,
    point: Vec3,
    dist: f32,
}

impl Plane {
    fn new(normal: Vec3, point: Vec3) -> Self {
        let dist = normal.dot(point);
        Self {
            normal,
            point,
            dist,
        }
    }

    fn distance<T: Into<Vec3>>(&self, point: T) -> f32 {
        let point = point.into();
        self.normal.dot(point) - self.dist
    }
}
impl TryFrom<&Mesh> for BSPTree {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> Result<Self, Self::Error> {
        let polygons = mesh.try_polygons()?;
        try_build_bsp_tree(&polygons, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::bool::bsp::{BSPNode, BSPTree};
    use crate::mesh::shape::cuboid::cube::Cube;
    use crate::mesh::HasMesh;
    use crate::mesh::shape::cone::Cone;
    use crate::mesh::shape::sphere::Sphere;
    use crate::turn_on_test_logs;

    #[test]
    fn smoke_test() {
        turn_on_test_logs();
        let fig = Cone::default();
        let mesh = fig.mesh();
        let bsp: BSPTree = mesh.try_into().unwrap();

        fn print_tree(node: &BSPNode, depth: usize) {
            match node {
                BSPNode::Leaf { polygons } => {
                    println!("{}Leaf: {:?}", " ".repeat(depth), polygons);
                }
                BSPNode::Node {
                    front,
                    back,
                    polygons,
                    plane,
                } => {
                    println!("{}Node: {:?}", " ".repeat(depth), polygons);
                    print_tree(front, depth + 1);
                    print_tree(back, depth + 1);
                }
            }
        }
        print_tree(&bsp.root, 0);

        // for node in bsp.iter_preorder() {
        //     println!("{:?}", node);
        // }
        //
        for node in bsp.iter_inorder() {
            println!("{:?}", node);
        }
    }
}
