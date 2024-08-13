use crate::mesh::material::Color;
use crate::mesh::normals::MeshNormals;
use crate::mesh::parts::BoundingBox;
use crate::mesh::tables::MeshTables;
use parts::Edge;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use parts::face::Face;
use parts::polygon::Polygon;
use parts::vertex::Vertex;

pub mod query;
pub mod material;
pub mod normals;
pub mod parts;
pub mod shape;
pub mod tables;
pub mod transform;
pub mod bool;

type MeshResult<T> = Result<T, MeshError>;

#[derive(Debug, Clone, PartialEq)]
pub enum MeshError {
    InvalidIndex(String),
    InvalidFaceType(String),
    WrongIntersection(String),
    Custom(String),
}

impl MeshError {
    fn idx_edge(v1: usize, v2: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the edge: {} - {}", v1, v2))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    faces: Vec<Face>,
    color: Color,
}
impl Mesh {
    pub fn from_vertices<V, F>(vertices: Vec<V>, faces: Vec<F>, color: Color) -> Self
    where
        V: Into<Vertex>,
        F: Into<Face>,
    {
        let vertices = vertices.into_iter().map(Into::into).collect();
        let faces: Vec<Face> = faces.into_iter().map(Into::into).collect();
        let edges: Vec<Edge> = dedup(
            faces
                .iter()
                .flat_map(|f| <&Face as Into<Vec<Edge>>>::into(f))
                .collect(),
        );
        Mesh {
            vertices,
            edges,
            faces,
            color,
        }
    }

    pub fn from_polygons(polygons: Vec<Polygon>, color: Color) -> Self {
        let mut vertices = Vec::new();
        let mut vertices_map = HashMap::new();
        let mut faces = Vec::new();
        for polygon in polygons.iter() {
            for v in polygon.vertices() {
                if !vertices_map.contains_key(v) {
                    vertices_map.insert(v, vertices.len());
                    vertices.push(v.clone());
                }
            }
        }

        for polygon in polygons.iter() {
            let vertices = polygon.vertices();

            let faces_poly = polygon.triangulate()
                .iter()
                .map(|p| {
                    p.vertices()
                        .iter()
                        .map(|v| vertices_map[v])
                        .collect::<Vec<_>>()
                })
                .map(|v| Face::Triangle(v[0], v[1], v[2]))
                .collect::<Vec<_>>();
            faces.extend(faces_poly);
        }

        Mesh::from_vertices(vertices, faces, color)
    }

}
impl Mesh {
    pub fn aabb(&self) -> BoundingBox {
        BoundingBox::from(self)
    }
    pub fn subdivide(&mut self) -> MeshResult<()> {
        let mut new_vertices = self.vertices.to_vec();
        let mut new_faces = Vec::new();
        let mut midpoint_cache = HashMap::new();

        for face in self.faces.iter() {
            let face_vertices = face.flatten();
            let mut mid_indices = Vec::new();
            for i in 0..face_vertices.len() {
                let key = if face_vertices[i] < face_vertices[(i + 1) % face_vertices.len()] {
                    (face_vertices[i], face_vertices[(i + 1) % face_vertices.len()])
                } else {
                    (face_vertices[(i + 1) % face_vertices.len()], face_vertices[i])
                };

                if let Some(&mid_index) = midpoint_cache.get(&key) {
                    mid_indices.push(mid_index);
                } else {
                    let mid_vertex = midpoint(
                        &self.vertices[face_vertices[i]],
                        &self.vertices[face_vertices[(i + 1) % face_vertices.len()]],
                    );
                    let mid_index = new_vertices.len();
                    new_vertices.push(mid_vertex);
                    midpoint_cache.insert(key, mid_index);
                    mid_indices.push(mid_index);
                }
            }

            match face {
                Face::Triangle(_, _, _) => {
                    new_faces.push(Face::Triangle(
                        face_vertices[0],
                        mid_indices[0],
                        mid_indices[2],
                    ));
                    new_faces.push(Face::Triangle(
                        face_vertices[1],
                        mid_indices[1],
                        mid_indices[0],
                    ));
                    new_faces.push(Face::Triangle(
                        face_vertices[2],
                        mid_indices[2],
                        mid_indices[1],
                    ));
                    new_faces.push(Face::Triangle(mid_indices[0], mid_indices[1], mid_indices[2]));
                }
                Face::Quad(_, _, _, _) => {
                    // For Quad, adapt the logic to handle quad subdivision
                }
            }
        }

        let new_vertices = new_vertices.into_iter().map(|v| v.normalize()).collect();
        *self = Mesh::from_vertices(new_vertices, new_faces, self.color.clone());
        Ok(())
    }
}
impl Mesh{
    pub fn try_tables(&self) -> MeshResult<MeshTables> {
        self.try_into()
    }
    pub fn try_normals(&self) -> MeshResult<MeshNormals> {
        self.try_into()
    }
    pub fn try_polygons(&self) -> MeshResult<Vec<Polygon>> {
        self.faces
            .iter()
            .map(|f| self.face_to_polygon(f))
            .collect::<Result<Vec<_>, _>>()
    }
}
impl Mesh {
    pub fn get(&self, idx: usize) -> MeshResult<&Vertex> {
        self.vertices
            .get(idx)
            .ok_or(MeshError::InvalidIndex("Invalid vertex index".to_string()))
    }




    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }
    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }


    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn face_to_polygon(&self, face: &Face) -> MeshResult<Polygon> {
        face.flatten()
            .iter()
            .map(|i| self.get(*i))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map(Polygon::new)
    }


}

pub trait HasMesh {
    fn mesh(&self) -> &Mesh;
    fn mesh_mut(&mut self) -> &mut Mesh;
}


impl<T: HasMesh> From<T> for Mesh {
    fn from(item: T) -> Self {
        item.mesh().clone()
    }
}

pub fn midpoint(vertex1: &Vertex, vertex2: &Vertex) -> Vertex {
    Vertex::new(
        (vertex1.x + vertex2.x) / 2.0,
        (vertex1.y + vertex2.y) / 2.0,
        (vertex1.z + vertex2.z) / 2.0,
    )
}
pub fn dedup<T: Hash + Eq>(items: Vec<T>) -> Vec<T> {
    items
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}



#[cfg(test)]
mod tests {
    use glam::Vec3;

    #[test]
    fn test() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let faces: Vec<Face> = vec![(0, 1, 2).into(), (0, 2, 3).into()];
        let mesh = Mesh::from_vertices(vertices, faces, Default::default());
        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.edges.len(), 4);
        assert_eq!(mesh.faces.len(), 2);
    }

    #[test]
    fn test_normals() {
        use super::*;
        let vertices = vec![[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0]];
        let faces: Vec<Face> = vec![(0, 1, 2).into(), (0, 2, 3).into()];
        let mesh = Mesh::from_vertices(vertices, faces, Default::default());
        let normals = mesh.try_normals().unwrap();

        assert_eq!(normals.get_normal(0), Ok(&Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(1), Ok(&Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(2), Ok(&Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(normals.get_normal(3), Ok(&Vec3::new(0.0, 0.0, 1.0)));
    }
}
