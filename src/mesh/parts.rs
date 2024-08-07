use crate::mesh::material::Color;
use crate::mesh::shape::cuboid::rect_cuboid::RectCuboid;
use crate::mesh::Mesh;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use face::FaceType;
use polygon::Polygon;
use vertex::Vertex;

pub mod vertex;
pub mod polygon;
pub mod face;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    Index(usize, usize),
    Vertex(Vertex, Vertex),
}

impl Edge {
    pub fn indexes(&self) -> Option<(usize, usize)> {
        match self {
            Edge::Index(a, b) => Some((*a, *b)),
            Edge::Vertex(a, b) => None,
        }
    }

    pub fn vertices(&self) -> Option<(Vertex, Vertex)> {
        match self {
            Edge::Index(_, _) => None,
            Edge::Vertex(a, b) => Some((*a, *b)),
        }
    }

    pub fn new_idx(a: usize, b: usize) -> Self {
        Edge::Index(a, b)
    }
    pub fn new_vtx(a: Vertex, b: Vertex) -> Self {
        Edge::Vertex(a, b)
    }
}

impl<V> From<(V, V)> for Edge
where
    V: Into<usize>,
{
    fn from(value: (V, V)) -> Self {
        Edge::new_idx(value.0.into(), value.1.into())
    }
}
pub type Idx = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BoundingBox {
    min_vertex: Vertex,
    max_vertex: Vertex,
}

impl From<(BoundingBox, BoundingBox)> for BoundingBox {
    fn from((lhs, rhs): (BoundingBox, BoundingBox)) -> Self {
        BoundingBox::merge(lhs, rhs)
    }
}

impl From<Vec<BoundingBox>> for BoundingBox {
    fn from(value: Vec<BoundingBox>) -> Self {
        value
            .into_iter()
            .fold(BoundingBox::default(), |acc, v| BoundingBox::merge(acc, v))
    }
}

impl BoundingBox {
    pub fn new(min_vertex: Vertex, max_vertex: Vertex) -> Self {
        Self {
            min_vertex,
            max_vertex,
        }
    }
    pub fn from_polygon(polygon: &Polygon) -> BoundingBox {
        let mut min_v = Vertex::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_v = Vertex::new(f32::MIN, f32::MIN, f32::MIN);
        for vertex in polygon.vertices().iter() {
            min_v.x = min_v.x.min(vertex.x);
            min_v.y = min_v.y.min(vertex.y);
            min_v.z = min_v.z.min(vertex.z);
            max_v.x = max_v.x.max(vertex.x);
            max_v.y = max_v.y.max(vertex.y);
            max_v.z = max_v.z.max(vertex.z);
        }
        Self {
            min_vertex: min_v,
            max_vertex: max_v,
        }
    }
    pub fn from_polygons(polygons: &Vec<Polygon>) -> BoundingBox {
        polygons
            .into_iter()
            .map(BoundingBox::from_polygon)
            .collect::<Vec<_>>()
            .into()
    }

    pub fn merge(lhs: BoundingBox, rhs: BoundingBox) -> Self {
        let min_vertex = Vertex::new(
            lhs.min_vertex.x.min(rhs.min_vertex.x),
            lhs.min_vertex.y.min(rhs.min_vertex.y),
            lhs.min_vertex.z.min(rhs.min_vertex.z),
        );
        let max_vertex = Vertex::new(
            lhs.max_vertex.x.max(rhs.max_vertex.x),
            lhs.max_vertex.y.max(rhs.max_vertex.y),
            lhs.max_vertex.z.max(rhs.max_vertex.z),
        );
        Self {
            min_vertex,
            max_vertex,
        }
    }

    pub fn to_rect_cuboid<C>(self, face_type: FaceType, color: C) -> RectCuboid
    where
        C: Into<Color>,
    {
        RectCuboid::create_bbox(self.min_vertex, self.max_vertex, face_type, color)
    }

    pub fn max(&self) -> &Vertex {
        &self.max_vertex
    }
    pub fn max_mut(&mut self) -> &mut Vertex {
        &mut self.max_vertex
    }
    pub fn min(&self) -> &Vertex {
        &self.min_vertex
    }
    pub fn min_mut(&mut self) -> &mut Vertex {
        &mut self.min_vertex
    }
}

impl From<&Mesh> for BoundingBox {
    fn from(value: &Mesh) -> Self {
        let mut min_v = Vertex::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_v = Vertex::new(f32::MIN, f32::MIN, f32::MIN);
        for vertex in value.vertices.iter() {
            min_v.x = min_v.x.min(vertex.x);
            min_v.y = min_v.y.min(vertex.y);
            min_v.z = min_v.z.min(vertex.z);
            max_v.x = max_v.x.max(vertex.x);
            max_v.y = max_v.y.max(vertex.y);
            max_v.z = max_v.z.max(vertex.z);
        }
        Self {
            min_vertex: min_v,
            max_vertex: max_v,
        }
    }
}
