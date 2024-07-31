use crate::mesh::bool::bsp::build::try_build_bsp_tree;
use crate::mesh::parts::{Face, Polygon, Vertex};
use crate::mesh::{Mesh, MeshError, MeshResult};
use glam::Vec3;
use rand::Rng;
use std::collections::HashSet;
use std::ops::Deref;
use crate::mesh::material::Color;

pub mod build;
pub mod query;

pub struct BSPTree {
    root: Box<BSPNode>,
}

impl Into<Mesh> for BSPTree {
    fn into(self) -> Mesh {
        self.mesh(Color::default())
    }
}

impl TryFrom<&Mesh> for BSPTree {
    type Error = MeshError;
    fn try_from(mesh: &Mesh) -> MeshResult<Self> {
        BSPTree::try_from_mesh(mesh, None)
    }
}

impl BSPTree {

    pub fn try_from_mesh(mesh: &Mesh, depth: Option<usize>) -> MeshResult<Self> {
        try_build_bsp_tree(&mesh.try_polygons()?, depth)
    }

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

    pub fn mesh(self, color:Color) -> Mesh {
        let polygons: HashSet<Polygon> = self
            .iter_preorder()
            .flat_map(|node| node.polygons().clone())
            .collect();
        Mesh::from_polygons(polygons.into_iter().collect(), color)
    }

    pub fn planes(&self, size: f32, color: Color) -> Vec<Mesh> {
        let mut meshes = vec![];
        for node in self.iter_preorder() {
            if let BSPNode::Node { plane, .. } = node {
                meshes.push(plane.to_mesh(size, color.clone()));
            }
        }
        meshes
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

    pub fn to_mesh(&self, size: f32, color: Color) -> Mesh {
        let right = self.normal.cross(Vec3::Y).normalize() * size;
        let up = self.normal.cross(right).normalize() * size;

        let vertices:Vec<Vertex> = vec![
            (self.point - right - up).into(),
            (self.point + right - up).into(),
            (self.point + right + up).into(),
            (self.point - right + up).into(),
        ];

        let faces = vec![
            Face::Triangle(0, 1, 2),
            Face::Triangle(2, 3, 0),
        ];

        Mesh::from_vertices(vertices, faces, color)
    }
}

#[cfg(test)]
mod tests {
    use crate::gpu::camera::position::CameraPosition;
    use crate::gpu::visualize;
    use crate::mesh::bool::bsp::{BSPNode, BSPTree};
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

    #[test]
    fn viz_test() {
        turn_on_test_logs();
        let fig = Cone::default();
        let mesh = fig.mesh();
        let bsp: BSPTree = mesh.try_into().unwrap();

        let orig_mesh = bsp.mesh(Default::default());
        let camera = CameraPosition::default();
        visualize(vec![orig_mesh,mesh.clone()], camera).unwrap();
    }
}
