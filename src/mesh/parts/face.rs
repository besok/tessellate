use std::hash::{Hash, Hasher};
use crate::mesh::parts::{Edge, Idx};

impl From<&Face> for Vec<Edge> {
    fn from(face: &Face) -> Self {
        match face {
            Face::Triangle(a, b, c) => vec![
                Edge::new_idx(*a, *b),
                Edge::new_idx(*b, *c),
                Edge::new_idx(*c, *a),
            ],
            Face::Quad(a, b, c, d) => vec![
                Edge::new_idx(*a, *b),
                Edge::new_idx(*b, *c),
                Edge::new_idx(*c, *d),
                Edge::new_idx(*d, *a),
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Triangle(Idx, Idx, Idx),
    Quad(Idx, Idx, Idx, Idx),
}

impl Face {
    pub fn flatten(&self) -> Vec<usize> {
        match self {
            Face::Triangle(a, b, c) => vec![*a, *b, *c],
            Face::Quad(a, b, c, d) => vec![*a, *b, *c, *d],
        }
    }
}

impl From<(usize, usize, usize, usize)> for Face {
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Face::Quad(value.0, value.1, value.2, value.3)
    }
}

impl From<(usize, usize, usize)> for Face {
    fn from(value: (usize, usize, usize)) -> Self {
        Face::Triangle(value.0, value.1, value.2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceType {
    Triangle,
    Quad,
}

impl Hash for FaceType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FaceType::Triangle => 0.hash(state),
            FaceType::Quad => 1.hash(state),
        }
    }
}

impl Default for FaceType {
    fn default() -> Self {
        FaceType::Triangle
    }
}