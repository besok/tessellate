use crate::mesh::parts::Face;
use glam::Vec3;

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

    fn distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.dist
    }
}

struct BSP {
    root: Box<BSPNode>,
}

enum BSPNode {
    Leaf(Face),
    Node {
        plane: Plane,
        front: Box<BSPNode>,
        back: Box<BSPNode>,
    },
}

struct BSPNode1 {
    plane: Plane,
    front: Option<Box<BSPNode>>,
    back: Option<Box<BSPNode>>,
    polygons: Vec<Face>,
}
