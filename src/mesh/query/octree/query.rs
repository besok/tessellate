use crate::mesh::query::octree::OctNode;

/// Iterator

pub struct OctreeIterator<'a> {
    stack: Vec<&'a OctNode>,
    only_leafs: bool,
}

impl<'a> OctreeIterator<'a> {
    pub fn new(root: &'a OctNode, only_leafs: bool) -> Self {
        let mut stack = Vec::new();
        stack.push(root);
        OctreeIterator { stack, only_leafs }
    }
}

impl<'a> Iterator for OctreeIterator<'a> {
    type Item = &'a OctNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                OctNode::Leaf { .. } => {
                    return Some(node)
                }
                OctNode::Node { children, .. } => {
                    for child in children.iter().rev() {
                        self.stack.push(child);
                    }
                    if !self.only_leafs {
                        return Some(node)
                    }
                }
            }
        }
        None
    }
}