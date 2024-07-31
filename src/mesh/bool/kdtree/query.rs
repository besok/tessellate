use crate::mesh::bool::kdtree::KDNode;

pub struct KDTreeIter<'a> {
    stack: Vec<&'a KDNode>,
}

impl<'a> KDTreeIter<'a> {
    pub fn new(root: &'a KDNode) -> Self {
        KDTreeIter {
            stack: vec![root],
        }
    }

    pub fn empty() -> Self {
        KDTreeIter {
            stack: vec![],
        }
    }
}

impl<'a> Iterator for KDTreeIter<'a> {
    type Item = &'a KDNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            match node {
                KDNode::Node { left, right, .. } => {
                    if let Some(right_node) = right {
                        self.stack.push(right_node);
                    }
                    if let Some(left_node) = left {
                        self.stack.push(left_node);
                    }
                }
                _ => {}
            }
            Some(node)
        } else {
            None
        }
    }
}



impl<'a> KDTreeIter<'a> {
    pub fn push_left(&mut self, node: &'a KDNode) {
        let mut current = Some(node);
        while let Some(node) = current {
            self.stack.push(node);
            current = match node {
                KDNode::Node { left, .. } => left.as_deref(),
                _ => None,
            };
        }
    }

    pub fn push_left_postorder(&mut self, node: &'a KDNode) {
        let mut current = Some(node);
        while let Some(node) = current {
            self.stack.push(node);
            current = match node {
                KDNode::Node { left, .. } => left.as_deref(),
                _ => None,
            };
        }
        if let Some(KDNode::Node { right, .. }) = self.stack.last() {
            if let Some(right_node) = right {
                self.push_left_postorder(right_node);
            }
        }
    }
}