mod geometry;

use crate::mesh::normals::calculate_normal;
use crate::mesh::parts::edge::Edge;
use crate::mesh::parts::vertex::{Vertex, Vertex2};
use crate::mesh::{MeshError, MeshResult};
use std::fmt::Display;
use crate::mesh::parts::polygon::geometry::{polys_tri_intersect, triangle_is_colinear, SimplexIntersection};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polygon {
    vertices: Vec<Vertex>,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }
}
impl Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Polygon({})",
            self.vertices
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl From<Vec<Vertex>> for Polygon {
    fn from(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }
}

impl Polygon {
    pub fn take(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }
    pub fn new(vertices: Vec<&Vertex>) -> Self {
        Self {
            vertices: vertices.into_iter().map(|v| v.clone()).collect(),
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn has_v(&self, v: &Vertex) -> bool {
        self.vertices.contains(v)
    }

    pub fn contains(&self, vertex: &Vertex) -> bool {
        self.edges().iter().any(|e| e.contains(vertex))
    }
    pub fn triangulate(&self) -> Vec<Polygon> {
        let vs = self.vertices();
        if vs.len() <= 3 {
            vec![self.clone()]
        } else {
            (1..vs.len() - 1)
                .map(|i| Polygon::new(vec![&vs[0], &vs[i], &vs[i + 1]]))
                .collect()
        }
    }
    pub fn centroid(&self) -> MeshResult<Vertex> {
        if self.vertices.is_empty() {
            Err(MeshError::Custom("empty polygon".to_string()))
        } else {
            let mut centroid = Vertex::default();
            for vertex in self.vertices.iter() {
                centroid = centroid + *vertex;
            }

            Ok(centroid * (1.0 / self.vertices.len() as f32))
        }
    }

    pub fn normal(&self) -> Vertex {
        calculate_normal(self.vertices()).into()
    }

    /// Calculate the winding number of a vertex with respect to the polygon
    /// The winding number is the number of times the polygon winds around the vertex
    /// The winding number is normalized to the range [0, 1]
    pub fn wnv(&self, vertex: &Vertex) -> f32 {
        let mut wn = 0.0;
        let v = vertex.clone();
        for e in self.edges().iter() {
            let (lhs, rhs) = e.vertices();
            let v1 = lhs - v;
            let v2 = rhs - v;

            wn += v1.cross(&v2).magnitude().atan2(v1.dot(&v2))
        }
        wn / (2.0 * std::f32::consts::PI)
    }

    /// Calculate the winding number trace value of a vertex with respect to the polygon
    /// The winding number trace value is the sum of the winding number of the polygon's edges
    pub fn wntv(&self, reference: &Vertex) -> f32 {
        let mut delta_wt = 0.0;
        let r = reference.clone();
        for e in self.edges().iter() {
            let (start, end) = e.vertices();
            delta_wt += calculate_segment_wntv(start.clone(), end.clone(), r.clone());
        }
        delta_wt
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.vertices
            .iter()
            .zip(self.vertices.iter().cycle().skip(1))
            .map(|(a, b)| Edge::new(a.clone(), b.clone()))
            .collect()
    }
    pub fn coincides(&self, other: &Polygon) -> bool {
        self.vertices.len() == other.vertices.len()
            && self.vertices.iter().all(|v| other.vertices.contains(v))
    }
    pub fn vertices_are_colinear(&self) -> bool {
        let ps = self.triangulate();
        ps.iter().any(|p| {
            let v0 = p.vertices()[0];
            let v1 = p.vertices()[1];
            let v2 = p.vertices()[2];
            triangle_is_colinear(&v0, &v1, &v2)
        })
    }

    pub fn intersects(&self, other: &Polygon) -> MeshResult<bool> {
        if self.vertices_are_colinear() || other.vertices_are_colinear() {
            Err(MeshError::Custom("colinear vertices".to_string()))
        } else {
            for self_tri in self.triangulate().iter() {
                for other_tri in other.triangulate().iter() {
                    if matches!(
                        polys_tri_intersect(self_tri, other_tri)?,
                        SimplexIntersection::Intersect
                    ) {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::material::Color;
    use crate::mesh::parts::polygon::Polygon;
    use crate::mesh::parts::vertex::Vertex;
    use crate::mesh::shape::icosahedron::Icosahedron;
    use crate::mesh::HasMesh;
    use crate::{mesh_edge, poly, v};

    #[test]
    fn intersects_coincides() {
        let p1 = poly!(ref &v!(), &v!(1,,), &v!(0.5, 1,));
        let p2 = poly!(ref &v!(), &v!(1,,), &v!(0.5, 1,));
        assert!(p1.intersects(&p2).unwrap());
    }

    #[test]
    fn intersects() {
        let p1 = poly!(ref &v!(), &v!(, 1,), &v!(1,,));
        let p2 = poly!(ref &v!(1.5,,), &v!(0.5,,), &v!(1.5, 1.5,));
        assert!(p1.intersects(&p2).unwrap());
    }

    #[test]
    fn test_wnv() {
        // Define a polygon
        let polygon = poly!(ref &v!(), &v!(1,,), &v!(0.5, 1,));

        // Define a test vertex
        let test_vertex = v!(0.5, 0.25,);

        // Calculate the winding number
        let winding_number = polygon.wnv(&test_vertex);

        assert!((winding_number - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_wntv() {
        let poly = Icosahedron::create(Vertex::default(), 1.0, Color::default());

        for p in poly.mesh().try_polygons().unwrap() {
            let wntv = p.wntv(&Vertex::default());

            println!("{wntv}");
        }
    }

    #[test]
    fn test_centroid() {
        let p = poly!(0,0,0; 3,3,3 ; 0,3,0);
        let c = p.centroid().unwrap();
        assert_eq!(c, v!(1.0, 2.0, 1.0));
    }

    #[test]
    fn test_intersects_sat() {
        let p1 = Polygon::new(vec![
            &Vertex::new(0.0, 0.0, 0.0),
            &Vertex::new(1.0, 0.0, 0.0),
            &Vertex::new(0.5, 1.0, 0.0),
        ]);
        let p2 = Polygon::new(vec![
            &Vertex::new(0.5, 0.5, 0.0),
            &Vertex::new(1.5, 0.5, 0.0),
            &Vertex::new(1.0, 1.5, 0.0),
        ]);
        assert!(p1.intersects(&p2).unwrap());

        let p3 = Polygon::new(vec![
            &Vertex::new(2.0, 2.0, 0.0),
            &Vertex::new(3.0, 2.0, 0.0),
            &Vertex::new(2.5, 3.0, 0.0),
        ]);
        assert!(!p1.intersects(&p3).unwrap());
    }
    #[test]
    fn test_intersects() {
        let p1 = poly!(-2.5, -2.5, 0; 2.5, -2.5, 0; 0, 0, 5);
        let p2 = poly!(2.5, 2.5, 0; 2.5, -2.5, 0; 0, 0, 5);

        assert!(p1.intersects(&p2).unwrap());
    }
}
