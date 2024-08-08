use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::mesh::query::sskdtree::SSKDNode;
use crate::mesh::parts::vertex::Vertex;

pub struct InOrderIter<'a> {
    stack: Vec<&'a SSKDNode>,
}

impl<'a> InOrderIter<'a> {
    pub fn new(root: &'a SSKDNode) -> Self {
        let mut stack = Vec::new();
        stack.push(root);
        InOrderIter { stack }
    }
}

impl<'a> Iterator for InOrderIter<'a> {
    type Item = &'a SSKDNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                SSKDNode::Leaf { .. } => {
                    return Some(node);
                }
                SSKDNode::Node { left, right, .. } => {
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
#[derive(Debug)]
pub struct Neighbor<'a> {
    pub node: &'a SSKDNode,
    pub distance: f32,
}

impl<'a> PartialEq for Neighbor<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<'a> Eq for Neighbor<'a> {}

impl<'a> PartialOrd for Neighbor<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl<'a> Ord for Neighbor<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.partial_cmp(&self.distance).unwrap()
    }
}

pub struct SSKDTreeNearestNeighborIter<'a> {
    stack: Vec<&'a SSKDNode>,
    target: &'a Vertex,
    max_dist: Option<f32>,
    heap: BinaryHeap<Neighbor<'a>>,
}

impl<'a> SSKDTreeNearestNeighborIter<'a> {
    pub fn new(root: &'a SSKDNode, target: &'a Vertex, max_dist: Option<f32>) -> Self {
        let mut iter = SSKDTreeNearestNeighborIter {
            stack: vec![root],
            target,
            max_dist,
            heap: BinaryHeap::new(),
        };
        iter.find_neighbors();
        iter
    }

    fn find_neighbors(&mut self) {
        while let Some(node) = self.stack.pop() {
            match node {
                SSKDNode::Leaf { polygons, .. } => {
                    for poly in polygons {
                        for vertex in poly.vertices() {
                            let dist = self.target.distance(vertex);
                            if self.max_dist.map_or(true, |max| dist <= max) {
                                self.heap.push(Neighbor { node, distance: dist });
                            }
                        }
                    }
                }
                SSKDNode::Node { left, right, .. } => {
                    if let Some(left) = left {
                        self.stack.push(left);
                    }
                    if let Some(right) = right {
                        self.stack.push(right);
                    }
                }
            }
        }
    }
}

impl<'a> Iterator for SSKDTreeNearestNeighborIter<'a> {
    type Item = Neighbor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}