use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex([f32; 3]);

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Vertex(value)
    }
}

impl From<[i32; 3]> for Vertex {
    fn from(value: [i32; 3]) -> Self {
        Vertex([value[0] as f32, value[1] as f32, value[2] as f32])
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for &val in &self.0 {
            val.to_bits().hash(state);
        }
    }
}

pub struct Edge(pub Vertex, pub Vertex);

impl<V> From<(V, V)> for Edge
where
    V: Into<Vertex>,
{
    fn from(value: (V, V)) -> Self {
        Edge(value.0.into(), value.1.into())
    }
}

pub type BorrowedEdge<'a> = (&'a Vertex, &'a Vertex);

pub type Idx = usize;

pub type VColor = [f32; 4];
pub type VNormal = [f32; 3];

pub enum Face {
    Triangle(Vertex, Vertex, Vertex),
    Quad(Vertex, Vertex, Vertex, Vertex),
}


pub enum Array34<T> {
    Array3([T; 3]),
    Array4([T; 4]),
}

impl From<[usize; 3]> for Array34<usize> {
    fn from(value: [usize; 3]) -> Self {
        Array34::Array3(value)
    }
}

impl From<[usize; 4]> for Array34<usize> {
    fn from(value: [usize; 4]) -> Self {
        Array34::Array4(value)
    }
}
