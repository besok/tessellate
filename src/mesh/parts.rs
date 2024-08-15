use crate::mesh::material::Color;
use crate::mesh::shape::cuboid::rect_cuboid::RectCuboid;
use crate::mesh::{Mesh, MeshError, MeshResult};
use face::FaceType;
use glam::{Mat2, Vec2, Vec3};
use polygon::Polygon;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Sub};
use vertex::Vertex;

pub mod face;
pub mod polygon;
pub mod vertex;
mod r#macro;

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

    /// Validate if the edge intersects with another edge
    pub fn is_intersected(&self, edge: &Edge) -> MeshResult<bool> {
        match (*self, *edge) {
            (Edge::Vertex(a1, b1), Edge::Vertex(a2, b2)) => {
                let diff_e1:Vec3 = (b1 - a1).into();
                let diff_e2:Vec3 = (b2 - a2).into();
                let cross = diff_e1.cross(diff_e2);

                if cross.length_squared() == 0.0 {
                    // Lines are parallel, no intersection unless they are collinear
                    return Ok(false);
                }

                // Parameterize lines a1 + t * de1 and a2 + u * de2 and solve for t and u
                let denom = cross.dot(cross);
                let diff:Vec3 = (a2 - a1).into();

                let t = diff.cross(diff_e2).dot(cross) / denom;
                let u = diff.cross(diff_e1).dot(cross) / denom;

                // Check if t and u are within [0, 1] which means the intersection lies within both segments
                Ok(t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 )

            }
            _ => Err(MeshError::WrongIntersection("Invalid intersection".to_string())),
        }
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

    pub fn intersects_polygon(&self, polygon: &Polygon) -> bool {
        for vertex in polygon.vertices() {
            if self.contains(vertex) {
                return true;
            }
        }
        false
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

    pub(crate) fn center(&self) -> Vertex {
        Vertex::new(
            (self.min_vertex.x + self.max_vertex.x) / 2.0,
            (self.min_vertex.y + self.max_vertex.y) / 2.0,
            (self.min_vertex.z + self.max_vertex.z) / 2.0,
        )
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let x_overlap =
            self.min_vertex.x <= other.max_vertex.x && self.max_vertex.x >= other.min_vertex.x;
        let y_overlap =
            self.min_vertex.y <= other.max_vertex.y && self.max_vertex.y >= other.min_vertex.y;
        let z_overlap =
            self.min_vertex.z <= other.max_vertex.z && self.max_vertex.z >= other.min_vertex.z;

        x_overlap && y_overlap && z_overlap
    }

    pub fn contains(&self, vertex: &Vertex) -> bool {
        let x_within = self.min_vertex.x <= vertex.x && vertex.x <= self.max_vertex.x;
        let y_within = self.min_vertex.y <= vertex.y && vertex.y <= self.max_vertex.y;
        let z_within = self.min_vertex.z <= vertex.z && vertex.z <= self.max_vertex.z;

        x_within && y_within && z_within
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
