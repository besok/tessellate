use crate::mesh::{MeshError, MeshResult};
use crate::mesh::parts::Edge;
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Polygon {
    vertices: Vec<Vertex>,
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

    pub fn wnv(&self, vertex: &Vertex) -> f32 {
        let mut wn = 0.0;
        let v = vertex.clone();
        for e in self.edges().iter() {
            if let Some((lhs, rhs)) = e.vertices() {
                let v1 = lhs - v;
                let v2 = rhs - v;

                let cross = v1.cross(&v2);
                let mag_cross = cross.magnitude();
                let Vertex{ x, y, z } = v1 * v2;
                let dot = v1.dot(&v2);
                let angle = mag_cross.atan2(dot);
                wn += angle
            }
        }
        wn / (4.0 * std::f32::consts::PI)
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.vertices
            .iter()
            .zip(self.vertices.iter().cycle().skip(1))
            .map(|(a, b)| Edge::new_vtx(a.clone(), b.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::parts::vertex::Vertex;
    use crate::mesh::parts::polygon::Polygon;

    #[test]
    fn test_wnv() {
        // Define a polygon
        let polygon = Polygon {
            vertices: vec![
                Vertex { x: 0.0, y: 0.0, z: 0.0 },
                Vertex { x: 5.0, y: 0.0, z: 0.0 },
                Vertex { x: 5.0, y: 5.0, z: 0.0 },
                Vertex { x: 0.0, y: 5.0, z: 0.0 },
            ],
        };

        // Define a test vertex
        let test_vertex = Vertex { x: 3.0, y: 3.0, z: 0.0 };

        // Calculate the winding number
        let winding_number = polygon.wnv(&test_vertex);

        // Assert the result
        assert_eq!(winding_number, 0.5);
    }
}