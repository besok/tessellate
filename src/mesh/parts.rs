use glam::Vec3;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl From<Vec3> for Vertex {
    fn from(value: Vec3) -> Self {
        Vertex {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl Into<Vec3> for Vertex {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
impl Into<Vec3> for &Vertex {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Mul<f32> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f32) -> Self::Output {
        Vertex {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Self) -> Self::Output {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex::new(0.0, 0.0, 0.0)
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { x, y, z }
    }
    pub fn flatten(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn normalize(&self) -> Vertex {
        <&Vertex as Into<Vec3>>::into(&self).normalize().into()
    }
}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Vertex {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<[i32; 3]> for Vertex {
    fn from(value: [i32; 3]) -> Self {
        Vertex {
            x: value[0] as f32,
            y: value[1] as f32,
            z: value[2] as f32,
        }
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge(pub Idx, pub Idx);

impl From<&Face> for Vec<Edge> {
    fn from(face: &Face) -> Self {
        match face {
            Face::Triangle(a, b, c) => vec![Edge(*a, *b), Edge(*b, *c), Edge(*c, *a)],
            Face::Quad(a, b, c, d) => vec![Edge(*a, *b), Edge(*b, *c), Edge(*c, *d), Edge(*d, *a)],
        }
    }
}

impl<V> From<(V, V)> for Edge
where
    V: Into<usize>,
{
    fn from(value: (V, V)) -> Self {
        Edge(value.0.into(), value.1.into())
    }
}
pub type Idx = usize;

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
        FaceType::Quad
    }
}
