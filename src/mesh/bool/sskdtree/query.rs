
use crate::mesh::bool::sskdtree::SSKDNode;


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
