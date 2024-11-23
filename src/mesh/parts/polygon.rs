pub mod intersection;

use crate::mesh::normals::calculate_normal;
use crate::mesh::parts::edge::Edge;
use crate::mesh::parts::polygon::intersection::{
    calculate_segment_wntv, point_in_triangle_3d, polys_tri_intersect, triangle_is_colinear,
    vertices_are_collinear, PointInSimplex, SimplexIntersection,
};
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{MeshError, MeshResult};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Triangle {
    pub v0: Vertex,
    pub v1: Vertex,
    pub v2: Vertex,
}

impl Triangle {
    pub fn new(v0: Vertex, v1: Vertex, v2: Vertex) -> Self {
        Self { v0, v1, v2 }
    }

    pub fn check_point(&self, p: &Vertex) -> PointInSimplex {
        point_in_triangle_3d(p, &self.v0, &self.v1, &self.v2)
    }

    pub fn vertices(&self) -> (Vertex, Vertex, Vertex) {
        (self.v0, self.v1, self.v2)
    }

}

impl TryFrom<Polygon> for Triangle {
    type Error = MeshError;

    fn try_from(value: Polygon) -> Result<Self, Self::Error> {
        let polys = value.triangulate();
        if polys.len() != 1 {
            Err(MeshError::Custom("Polygon is not a triangle".to_string()))
        } else {
            Ok(Self::new(polys[0].vertices[0], polys[0].vertices[1], polys[0].vertices[2]))
        }
    }
}
/// Represents a polygonal surface in a mesh, typically defined by three or more vertices.
///
/// # Examples
///
/// ```
/// use tessellate::mesh::parts::polygon::Polygon;
/// use tessellate::mesh::parts::vertex::Vertex;
///
/// // Creating a polygon with three vertices
/// let v0 = Vertex::new(0.0, 0.0, 0.0);
/// let v1 = Vertex::new(1.0, 0.0, 0.0);
/// let v2 = Vertex::new(0.0, 1.0, 0.0);
/// let triangle = Polygon::new(vec![v0, v1, v2]);
///
/// // Creating a polygon with four vertices
/// let v3 = Vertex::new(1.0, 1.0, 0.0);
/// let quad:Polygon = vec![v0, v1, v2, v3].into();
/// ```
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
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }
    pub fn new_ref(vertices: Vec<&Vertex>) -> Self {
        Self {
            vertices: vertices.into_iter().map(|v| v.clone()).collect(),
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
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
                .map(|i| Polygon::new_ref(vec![&vs[0], &vs[i], &vs[i + 1]]))
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

    /// Check if the polygon intersects another polygon
    pub fn intersects(&self, other: &Polygon) -> MeshResult<bool> {
        if vertices_are_collinear(&self) || vertices_are_collinear(&other) {
            return Err(MeshError::Custom("collinear vertices".to_string()));
        }

        if self.coincides(other) {
            Ok(true)
        } else {
            for self_tri in self.triangulate().iter() {
                for other_tri in other.triangulate().iter() {
                    if polys_tri_intersect(self_tri, other_tri)?.intersect() {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }
    }
    pub fn intersects_precise(&self, other: &Polygon) -> MeshResult<SimplexIntersection> {
        if vertices_are_collinear(&self) || vertices_are_collinear(&other) {
            return Err(MeshError::Custom("collinear vertices".to_string()));
        }

        for self_tri in self.triangulate().iter() {
            for other_tri in other.triangulate().iter() {
                match polys_tri_intersect(self_tri, other_tri)? {
                    SimplexIntersection::DoNotIntersect => continue,
                    e => return Ok(e),
                }
            }
        }
        Ok(SimplexIntersection::DoNotIntersect)
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
    use crate::{poly, v};

    #[test]
    fn intersects_coincides() {
        let p1 = poly!(ref &v!(), &v!(1,0,0), &v!(0.5, 1,));
        let p2 = poly!(ref &v!(), &v!(1,0,0), &v!(0.5, 1,));
        assert!(p1.intersects(&p2).unwrap());
    }

    #[test]
    fn intersects() {
        let p1 = poly!(ref &v!(0,0,0), &v!(0, 1,0), &v!(1,0,0));
        let p2 = poly!(ref &v!(1.5,0,0), &v!(0.5,0,0), &v!(1.5, 1.5,0));
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
        let p1 = Polygon::new_ref(vec![
            &Vertex::new(0.0, 0.0, 0.0),
            &Vertex::new(1.0, 0.0, 0.0),
            &Vertex::new(0.5, 1.0, 0.0),
        ]);
        let p2 = Polygon::new_ref(vec![
            &Vertex::new(0.5, 0.5, 0.0),
            &Vertex::new(1.5, 0.5, 0.0),
            &Vertex::new(1.0, 1.5, 0.0),
        ]);
        assert!(p1.intersects(&p2).unwrap());

        let p3 = Polygon::new_ref(vec![
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
