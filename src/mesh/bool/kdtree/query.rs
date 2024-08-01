use std::cmp::Ordering;
use crate::mesh::bool::kdtree::KDNode;
use crate::mesh::parts::Vertex;
use std::collections::BinaryHeap;
use log::info;

#[derive(Debug, Clone)]
pub struct Neighbour {
    pub node: Box<KDNode>,
    pub distance: f32,
}

impl Neighbour {
    pub fn new(node: Box<KDNode>, distance: f32) -> Self {
        Neighbour { node, distance }
    }
}


impl  Eq for Neighbour  {}

impl PartialEq for Neighbour  {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Neighbour  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl  Ord for Neighbour  {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap_or(Ordering::Equal)
    }
}

pub struct KDTreeNearestNeighborIter<'a> {
    target: &'a Vertex,
    heap: BinaryHeap<Neighbour>,
}

impl<'a> KDTreeNearestNeighborIter<'a> {
    pub fn new(root: &'a KDNode, target: &'a Vertex) -> Self {
        let mut heap = BinaryHeap::new();
        heap.push(Neighbour::new(Box::new(root.clone()), root.point().distance(target)));
        KDTreeNearestNeighborIter { target, heap }
    }
}

impl<'a> Iterator for KDTreeNearestNeighborIter<'a> {
    type Item = Neighbour ;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some( Neighbour{ node, distance}) = self.heap.pop() {

            match *node {
                KDNode::Leaf { .. } => {
                    return Some(Neighbour::new(node, distance));
                }
                KDNode::Node {
                    point, ref left, ref right, ..
                } => {
                    let dist = self.target.distance(&point);
                    if let Some(left_node) = left {
                        self.heap.push(Neighbour::new(left_node.clone(), dist));
                    }
                    if let Some(right_node) = right {
                        self.heap.push( Neighbour::new(right_node.clone(), dist));
                    }
                }
            }
        }
        None
    }
}

pub struct InOrderIter<'a> {
    stack: Vec<&'a KDNode>,
}

impl<'a> InOrderIter<'a> {
    pub fn new(root: &'a KDNode) -> Self {
        let mut stack = Vec::new();
        stack.push(root);
        InOrderIter { stack }
    }
}

impl<'a> Iterator for InOrderIter<'a> {
    type Item = &'a KDNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                KDNode::Leaf { .. } => {
                    return Some(node);
                }
                KDNode::Node { left, right, .. } => {
                    if let Some(right) = right {
                        self.stack.push(right);
                    }
                    if let Some(left) = left {
                        self.stack.push(left);
                    }
                }
            }
        }
        None
    }
}
