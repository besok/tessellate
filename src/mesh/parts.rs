use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex{
    pub(crate) x:f32,
    pub(crate) y:f32,
    pub(crate) z:f32
}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Vertex{
            x:value[0],
            y:value[1],
            z:value[2]
        }
    }
}

impl From<[i32; 3]> for Vertex {
    fn from(value: [i32; 3]) -> Self {
        Vertex{
            x:value[0] as f32,
            y:value[1] as f32,
            z:value[2] as f32

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge(pub Idx, pub Idx);


impl<V> From<(V, V)> for Edge
where
    V: Into<usize>,
{
    fn from(value: (V, V)) -> Self {
        Edge(value.0.into(), value.1.into())
    }
}
pub type Idx = usize;

pub enum Face {
    Triangle(Idx, Idx, Idx),
    Quad(Idx, Idx, Idx, Idx),
}

impl<T: Into<usize>> From<Array34<T>> for Face {
    fn from(value: Array34<T>) -> Self {
        match value {
            Array34::Array3([v1, v2, v3]) => Face::Triangle(v1.into(), v2.into(), v3.into()),
            Array34::Array4([v1, v2, v3, v4]) => {
                Face::Quad(v1.into(), v2.into(), v3.into(), v4.into())
            }
        }
    }
}

pub enum Array34<T = Idx> {
    Array3([T; 3]),
    Array4([T; 4]),
}

impl From<[usize; 3]> for Array34 {
    fn from(value: [usize; 3]) -> Self {
        Array34::Array3(value)
    }
}

impl From<[usize; 4]> for Array34 {
    fn from(value: [usize; 4]) -> Self {
        Array34::Array4(value)
    }
}
