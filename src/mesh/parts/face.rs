use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::Idx;
use std::hash::{Hash, Hasher};

impl From<&Face> for Vec<MeshEdge> {
    fn from(face: &Face) -> Self {
        match face {
            Face::Triangle(a, b, c) => vec![
                MeshEdge::new(*a, *b),
                MeshEdge::new(*b, *c),
                MeshEdge::new(*c, *a),
            ],
            Face::Quad(a, b, c, d) => vec![
                MeshEdge::new(*a, *b),
                MeshEdge::new(*b, *c),
                MeshEdge::new(*c, *d),
                MeshEdge::new(*d, *a),
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
    pub fn new3(a: Idx, b: Idx, c: Idx) -> Self {
        Face::Triangle(a, b, c)
    }

    pub fn new4(a: Idx, b: Idx, c: Idx, d: Idx) -> Self {
        Face::Quad(a, b, c, d)
    }

    pub fn with_offset(&self, offset: usize) -> Face {
        match self {
            Face::Triangle(a, b, c) => Face::Triangle(a + offset, b + offset, c + offset),
            Face::Quad(a, b, c, d) => Face::Quad(a + offset, b + offset, c + offset, d + offset),
        }
    }

    pub fn edges(&self) -> Vec<MeshEdge> {
        match self {
            Face::Triangle(a, b, c) => {
                vec![
                    MeshEdge::new(*a, *b),
                    MeshEdge::new(*b, *c),
                    MeshEdge::new(*c, *a),
                ]
            }
            Face::Quad(a, b, c, d) => {
                vec![
                    MeshEdge::new(*a, *b),
                    MeshEdge::new(*b, *c),
                    MeshEdge::new(*c, *d),
                    MeshEdge::new(*d, *a),
                ]
            }
        }
    }

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
