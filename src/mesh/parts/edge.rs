use std::hash::Hash;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{MeshResult};
use glam::Vec3;

/// Mesh edge
/// The structure to store the edge of a mesh
/// The edge can be represented by indexes or vertices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshEdge(pub usize, pub usize);

impl MeshEdge {
    pub fn indexes(&self) -> Option<(usize, usize)> {
        Some((self.0, self.1))
    }

    pub fn new(a: usize, b: usize) -> Self {
        MeshEdge(a, b)
    }

    pub fn inv(&self) -> Self {
        MeshEdge(self.1, self.0)
    }
}

impl<V> From<(V, V)> for MeshEdge
where
    V: Into<usize>,
{
    fn from(value: (V, V)) -> Self {
        MeshEdge::new(value.0.into(), value.1.into())
    }
}

/// Edge
/// The structure to store an edge
/// The edge is represented by two vertices
#[derive(Debug, Clone, Copy, Hash)]
pub struct Edge {
    pub a: Vertex,
    pub b: Vertex,
}

impl<V:Into<Vertex>> From<(V,V)> for Edge {
    fn from(value: (V, V)) -> Self {
        Edge::new(value.0.into(), value.1.into())
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl Eq for Edge {}

impl Edge {
    pub fn new(a: Vertex, b: Vertex) -> Self {
        Self { a, b }
    }

    pub fn vertices(&self) -> (Vertex, Vertex) {
        (self.a, self.b)
    }

    pub fn contains(&self, vertex: &Vertex) -> bool {
        let diff_v = self.b - self.a;
        let diff_p = *vertex - self.a;
        diff_v.cross(&diff_p).length_squared() == 0.0
            && diff_v.dot(&diff_p) >= 0.0
            && diff_v.dot(&diff_p) <= diff_v.dot(&diff_v)
    }
    /// Find the overlapping segment between two edges
    /// Returns None if the edges are not collinear
    /// Returns the overlapping segment if the edges are collinear
    pub fn find_collinear_segment(&self, edge: &Edge) -> Option<Edge> {
        match (self, edge) {
            (Edge { a: a1, b: b1 }, Edge { a: a2, b: b2 }) => {
                let diff_e1: Vec3 = (*b1 - *a1).into();
                let diff_e2: Vec3 = (*b2 - *a2).into();
                let cross = diff_e1.cross(diff_e2);

                if cross.length_squared() != 0.0 {
                    // Edges are not parallel
                    None
                } else {
                    // Check if the edges are collinear
                    let diff = *a2 - *a1;
                    let diff_e1: Vertex = diff_e1.into();
                    if diff.cross(&diff_e1).length_squared() != 0.0 {
                        None
                    } else {
                        // Find the overlapping segment
                        let t0: f32 = 0.0;
                        let t1: f32 = 1.0;
                        let u0 = (*a2 - *a1).dot(&diff_e1) / diff_e1.dot(&diff_e1);
                        let u1 = (*b2 - *a1).dot(&diff_e1) / diff_e1.dot(&diff_e1);

                        let t_min = t0.max(u0.min(u1));
                        let t_max = t1.min(u0.max(u1));

                        if t_min <= t_max {
                            let start = *a1 + diff_e1 * t_min;
                            let end = *a1 + diff_e1 * t_max;
                            Some(Edge::new(start.into(), end.into()))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    /// Validate if the edge intersects with another edge
    pub fn is_intersected(&self, edge: &Edge) -> MeshResult<bool> {
        if self == edge {
            Ok(true)
        } else {
            match (self, edge) {
                (Edge { a: a1, b: b1 }, Edge { a: a2, b: b2 }) => {
                    let diff_e1: Vec3 = (*b1 - *a1).into();
                    let diff_e2: Vec3 = (*b2 - *a2).into();
                    let cross = diff_e1.cross(diff_e2);

                    if cross.length_squared() == 0.0 {
                        // Lines are parallel, no intersection unless they are collinear
                        return Ok(false);
                    }

                    // Parameterize lines a1 + t * de1 and a2 + u * de2 and solve for t and u
                    let denom = cross.dot(cross);
                    let diff: Vec3 = (*a2 - *a1).into();

                    let t = diff.cross(diff_e2).dot(cross) / denom;
                    let u = diff.cross(diff_e1).dot(cross) / denom;
                    // Check if t and u are within [0, 1] which means the intersection lies within both segments
                    Ok(t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0)
                }
            }
        }
    }

    pub fn find_intersection(&self, edge: &Edge) -> Option<Vertex> {
        match (self, edge) {
            (Edge { a: a1, b: b1 }, Edge { a: a2, b: b2 }) => {
                let diff_e1: Vec3 = (*b1 - *a1).into();
                let diff_e2: Vec3 = (*b2 - *a2).into();
                let cross = diff_e1.cross(diff_e2);

                if cross.length_squared() == 0.0 {
                    // Lines are parallel, no intersection unless they are collinear
                    return None;
                }

                // Parameterize lines a1 + t * de1 and a2 + u * de2 and solve for t and u
                let denom = cross.dot(cross);
                let diff: Vec3 = (*a2 - *a1).into();

                let t = diff.cross(diff_e2).dot(cross) / denom;
                let u = diff.cross(diff_e1).dot(cross) / denom;
                // Check if t and u are within [0, 1] which means the intersection lies within both segments
                if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
                    Some(*a1 + <Vec3 as Into<Vertex>>::into((diff_e1 * t).into()))
                } else {
                    None
                }
            }
        }
    }
}
