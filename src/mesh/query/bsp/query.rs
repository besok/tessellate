use IterNode::{Unvisited, Visited};
use crate::mesh::query::bsp::BSPNode;

pub struct BspPreorderIterator<'a> {
    stack: Vec<&'a BSPNode>,
}

impl<'a> BspPreorderIterator<'a> {
    pub fn new(root: &'a BSPNode) -> Self {
        Self {
            stack: vec![root],
        }
    }

}

impl<'a> Iterator for BspPreorderIterator<'a> {
    type Item = &'a BSPNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            match node {
                BSPNode::Leaf { .. } => {}
                BSPNode::Node { front, back, .. } => {
                    self.stack.push(back);
                    self.stack.push(front);
                }
            }
            Some(node)
        } else {
            None
        }
    }
}

enum IterNode<'a> {
    Visited(&'a BSPNode),
    Unvisited(&'a BSPNode),
}

pub struct InOrderIterator<'a> {
    stack: Vec<IterNode<'a>>,
}

impl<'a> InOrderIterator<'a> {
    pub fn new(root: &'a BSPNode) -> Self {
        Self {
            stack: vec![Unvisited(root)],
        }
    }
}

impl<'a> Iterator for InOrderIterator<'a> {
    type Item = &'a BSPNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                Visited(node) => return Some(node),
                Unvisited(node) => match node {
                    BSPNode::Leaf { .. } => return Some(node),
                    BSPNode::Node { front, back, .. } => {
                        self.stack.push(Unvisited(back));
                        self.stack.push(Visited(node));
                        self.stack.push(Unvisited(front));
                    }
                },
            }
        }
        None
    }
}

pub struct PostOrderIterator<'a> {
    stack: Vec<IterNode<'a>>,
}

impl<'a> PostOrderIterator<'a> {
    pub fn new(root: &'a BSPNode) -> Self {
        Self {
            stack: vec![Unvisited(root)],
        }
    }
}

impl<'a> Iterator for PostOrderIterator<'a> {
    type Item = &'a BSPNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            match node {
                Visited(node) => return Some(node),
                Unvisited(node) => match node {
                    BSPNode::Leaf { .. } => return Some(node),
                    BSPNode::Node { front, back, .. } => {
                        self.stack.push(Visited(node));
                        self.stack.push(Unvisited(back));
                        self.stack.push(Unvisited(front));
                    }
                },
            }
        }
        None
    }
}