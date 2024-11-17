use crate::mesh::attributes::{Attributes, MeshType};
use crate::mesh::material::Color;
use crate::mesh::normals::MeshNormals;
use crate::mesh::parts::edge::Edge;
use crate::mesh::tables::MeshTables;
use parts::bbox::BoundingBox;
use parts::edge::MeshEdge;
use parts::face::Face;
use parts::polygon::Polygon;
use parts::vertex::Vertex;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::num::TryFromIntError;

pub mod attributes;
pub mod bool;
pub mod material;
pub mod normals;
pub mod parts;
mod properties;
pub mod query;
pub mod shape;
pub mod tables;
pub mod transform;

pub type MeshResult<T> = Result<T, MeshError>;

#[derive(Debug, Clone, PartialEq)]
pub enum MeshError {
    InvalidIndex(String),
    InvalidFaceType(String),
    WrongIntersection(String),
    WrongMesh(String),
    Custom(String),
}

impl From<TryFromIntError> for MeshError {
    fn from(value: TryFromIntError) -> Self {
        MeshError::Custom(value.to_string())
    }
}

impl From<Infallible> for MeshError {
    fn from(_value: Infallible) -> Self {
        MeshError::Custom("Infallible".to_string())
    }
}

impl Display for MeshError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MeshError {
    pub fn idx_edge(v1: usize, v2: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the edge: {} - {}", v1, v2))
    }
    pub fn idx_vertex(v: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the vertex: {}", v))
    }
}

#[derive(Default, Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<MeshEdge>,
    faces: Vec<Face>,
    color: Color,
    attributes: Attributes,
}
impl Mesh {
    /// Creates a new `Mesh` from vertices and faces.
    ///
    /// # Parameters
    ///
    /// * `vertices` - A vector of vertices to be included in the mesh.
    /// * `faces` - A vector of faces to be included in the mesh.
    /// * `color` - The color of the mesh.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance containing the specified vertices and faces.
    pub fn from_vertices<V, F>(vertices: Vec<V>, faces: Vec<F>, color: Color) -> Self
    where
        V: Into<Vertex>,
        F: Into<Face>,
    {
        let vertices = vertices.into_iter().map(Into::into).collect();
        let faces: Vec<Face> = faces.into_iter().map(Into::into).collect();
        let edges: Vec<MeshEdge> = dedup(
            faces
                .iter()
                .flat_map(|f| <&Face as Into<Vec<MeshEdge>>>::into(f))
                .collect(),
        );
        Mesh {
            vertices,
            edges,
            faces,
            color,
            attributes: Attributes::default(),
        }
    }

    /// Creates a new `Mesh` from a vector of polygons.
    ///
    /// This function takes a vector of polygons and a color, and constructs a new `Mesh` instance.
    /// It first extracts unique vertices from the polygons and maps them to indices. Then, it
    /// triangulates each polygon and converts them into faces. Finally, it creates a new `Mesh`
    /// instance using the vertices and faces.
    ///
    /// # Parameters
    ///
    /// * `polygons` - A vector of polygons to be included in the mesh.
    /// * `color` - The color of the mesh.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance containing the specified polygons.

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
            let _vertices = polygon.vertices();

            let faces_poly = polygon
                .triangulate()
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

    /// Creates a new `Mesh` representing a cloud of vertices.
    ///
    /// # Parameters
    ///
    /// * `vertices` - A vector of vertices to be included in the cloud.
    /// * `vert_size` - The size of each vertex in the cloud.
    /// * `color` - The color of the mesh.
    ///
    /// # Returns
    ///
    /// A new `Mesh` instance representing the cloud of vertices.
    pub fn cloud<V>(vertices: Vec<V>, vert_size: usize, color: Color) -> Self
    where
        V: Into<Vertex>,
    {
        let vertices: Vec<_> = vertices.into_iter().map(Into::into).collect();
        Mesh {
            vertices,
            color,
            edges: vec![],
            faces: vec![],
            attributes: Attributes::new(MeshType::Cloud(vert_size)),
        }
    }

    pub fn lines(lines: Vec<Edge>, color: Color) -> Self {
        let mut vertices_map = HashMap::new();
        let mut vertices = Vec::new();

        for Edge { a, b } in lines.iter() {
            if !vertices_map.contains_key(a) {
                vertices_map.insert(a, vertices.len());
                vertices.push(a.clone());
            }
            if !vertices_map.contains_key(b) {
                vertices_map.insert(b, vertices.len());
                vertices.push(b.clone());
            }
        }

        let mesh_edges = lines
            .iter()
            .map(|Edge { a, b }| MeshEdge::new(vertices_map[a], vertices_map[b]))
            .collect();

        Mesh {
            vertices,
            edges: mesh_edges,
            faces: vec![],
            color,
            attributes: Attributes::new(MeshType::Lines),
        }
    }
}
impl Mesh {
    pub fn is_cloud(&self) -> bool {
        self.attributes.mesh_type().is_cloud()
    }
    pub fn is_lines(&self) -> bool {
        self.attributes.mesh_type().is_lines()
    }
    pub fn is_polygons(&self) -> bool {
        self.attributes.mesh_type().is_polygons()
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    pub fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    pub fn aabb(&self) -> BoundingBox {
        BoundingBox::from(self)
    }

    /// Subdivide the mesh
    /// The subdivision is based on the midpoint of the edges
    /// The new vertices are normalized
    /// The new faces are created based on the subdivision
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
impl Mesh {
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
    pub fn try_edges(&self) -> MeshResult<Vec<Edge>> {
        let mut edges = Vec::new();
        for MeshEdge(a, b) in self.edges.iter() {
            let v1 = self.get(*a)?;
            let v2 = self.get(*b)?;
            let edge = Edge::new(v1.clone(), v2.clone());
            edges.push(edge);
        }
        Ok(edges)
    }
    pub fn query(&self) -> query::MeshQuery {
        query::MeshQuery::from(self)
    }
}

impl Mesh {
    pub fn union<T: Into<Mesh>>(&self, other: T) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::Union, None)
    }
    pub fn union_with<T: Into<Mesh>>(&self, other: T, color: Option<Color>) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::Union, color)
    }
    pub fn intersection_with<T: Into<Mesh>>(
        &self,
        other: T,
        color: Option<Color>,
    ) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::Intersection, color)
    }
    pub fn intersection<T: Into<Mesh>>(&self, other: T) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::Intersection, None)
    }
    pub fn difference_with<T: Into<Mesh>>(
        &self,
        other: T,
        color: Option<Color>,
    ) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::Difference, color)
    }
    pub fn difference<T: Into<Mesh>>(&self, other: T) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::Difference, None)
    }

    pub fn sym_difference_with<T: Into<Mesh>>(
        &self,
        other: T,
        color: Option<Color>,
    ) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::SymmetricDifference, color)
    }
    pub fn sym_difference<T: Into<Mesh>>(&self, other: T) -> MeshResult<Mesh> {
        bool::perform_bool(self, &other.into(), bool::BoolType::SymmetricDifference, None)
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
    pub fn edges(&self) -> &Vec<MeshEdge> {
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

    pub fn props(&self) -> properties::MeshProperties {
        properties::MeshProperties::new(self)
    }

    fn face_to_polygon(&self, face: &Face) -> MeshResult<Polygon> {
        face.flatten()
            .iter()
            .map(|i| self.get(*i))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map(Polygon::new_ref)
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
        println!("{:?}", mesh.edges);
        assert_eq!(mesh.edges.len(), 6);
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
