use crate::mesh::parts::vertex::Vertex;
use crate::mesh::parts::Edge;
use crate::mesh::{MeshError, MeshResult};

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

impl From<Vec<Vertex>> for Polygon {
    fn from(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }
}

impl Polygon {
    pub fn new(vertices: Vec<&Vertex>) -> Self {
        Self {
            vertices: vertices.into_iter().map(|v| v.clone()).collect(),
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
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

    /// Calculate the winding number of a vertex with respect to the polygon
    /// The winding number is the number of times the polygon winds around the vertex
    /// The winding number is normalized to the range [0, 1]
    pub fn wnv(&self, vertex: &Vertex) -> f32 {
        let mut wn = 0.0;
        let v = vertex.clone();
        for e in self.edges().iter() {
            if let Some((lhs, rhs)) = e.vertices() {
                let v1 = lhs - v;
                let v2 = rhs - v;

                wn += v1.cross(&v2).magnitude().atan2(v1.dot(&v2))
            }
        }
        wn / (2.0 * std::f32::consts::PI)
    }

    /// Calculate the winding number trace value of a vertex with respect to the polygon
    /// The winding number trace value is the sum of the winding number of the polygon's edges
    pub fn wntv(&self, reference: &Vertex) -> f32 {
        let mut delta_wt = 0.0;
        let r = reference.clone();
        for e in self.edges().iter() {
            if let Some((start, end)) = e.vertices() {
                delta_wt += calculate_segment_wntv(start.clone(), end.clone(), r.clone());
            }
        }
        delta_wt
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.vertices
            .iter()
            .zip(self.vertices.iter().cycle().skip(1))
            .map(|(a, b)| Edge::new_vtx(a.clone(), b.clone()))
            .collect()
    }

    pub fn intersects(&self, other: &Polygon) -> MeshResult<bool> {
        for e1 in self.edges().iter() {
            for e2 in other.edges().iter() {
                if e1.intersects(e2)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

fn calculate_segment_wntv(start: Vertex, end: Vertex, reference: Vertex) -> f32 {
    let cross = (end - start).cross(&(reference - start));
    if cross.z > 0.0 {
        1.0
    } else {
        -1.0
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

    #[test]
    fn test_wnv() {
        // Define a polygon
        let polygon = Polygon {
            vertices: vec![
                Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vertex {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vertex {
                    x: 0.5,
                    y: 1.0,
                    z: 0.0,
                },
            ],
        };

        // Define a test vertex
        let test_vertex = Vertex {
            x: 0.5,
            y: 0.25,
            z: 0.0,
        };

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
}
