use crate::mesh::attributes::{Attributes, MeshType};
use crate::mesh::distance::distance_between_surfaces;
use crate::mesh::material::Color;
use crate::mesh::normals::MeshNormals;
use crate::mesh::parts::edge::Edge;
use crate::mesh::subdivision::by_loop;
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
pub mod distance;
pub mod material;
pub mod normals;
pub mod parts;
pub mod properties;
pub mod query;
pub mod shape;
pub mod subdivision;
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

impl From<&str> for MeshError {
    fn from(value: &str) -> Self {
        if value.contains("index") {
            MeshError::InvalidIndex(value.to_string())
        } else if value.contains("face") {
            MeshError::InvalidFaceType(value.to_string())
        } else if value.contains("intersection") {
            MeshError::WrongIntersection(value.to_string())
        } else if value.contains("mesh") {
            MeshError::WrongMesh(value.to_string())
        } else {
            MeshError::Custom(value.to_string())
        }
    }
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
    pub fn idx_face(v1: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the face: {}", v1))
    }
    pub fn idx_vertex(v: usize) -> Self {
        MeshError::InvalidIndex(format!("Invalid index for the vertex: {}", v))
    }
    pub fn wrong_vert(v: &Vertex) -> Self {
        MeshError::InvalidIndex(format!("Invalid vertex: {:?}", v))
    }
}

/// Represents a 3D mesh consisting of vertices, edges, and faces.
///
/// The `Mesh` struct is the core data structure for representing 3D meshes in Tessellate.
/// It contains vertices, edges, faces, color, and attributes.
///
/// # Fields
///
/// * `vertices` - A vector of vertices in the mesh.
/// * `edges` - A vector of edges in the mesh.
/// * `faces` - A vector of faces in the mesh.
/// * `attributes` - Additional attributes associated with the mesh.
///
/// # Example
///
/// ```
/// use tessellate::mesh::attributes::Attributes;
/// use tessellate::mesh::parts::vertex::Vertex;
/// use tessellate::mesh::parts::face::Face;
/// use tessellate::mesh::Mesh;
/// let vertices = vec![
///     Vertex::new(0.0, 0.0, 0.0),
///     Vertex::new(1.0, 0.0, 0.0),
///     Vertex::new(1.0, 1.0, 0.0),
///     Vertex::new(0.0, 1.0, 0.0),
/// ];
/// let faces = vec![
///     Face::Triangle(0, 1, 2),
///     Face::Triangle(0, 2, 3),
/// ];
/// let mesh = Mesh::from_vertices(vertices, faces, Attributes::default());
/// ```
#[derive(Default, Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    edges: Vec<MeshEdge>,
    faces: Vec<Face>,
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
    pub fn from_vertices<V, F>(vertices: Vec<V>, faces: Vec<F>, attributes: Attributes) -> Self
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
            attributes,
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

    pub fn from_polygons(polygons: Vec<Polygon>, attributes: Attributes) -> Self {
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

        Mesh::from_vertices(vertices, faces, attributes)
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
    pub fn cloud<V>(vertices: Vec<V>, vert_size: usize, attributes: Attributes) -> Self
    where
        V: Into<Vertex>,
    {
        let vertices: Vec<_> = vertices.into_iter().map(Into::into).collect();
        let mut attributes = attributes;
        attributes.set_mesh_type(MeshType::Cloud(vert_size));
        Mesh {
            vertices,
            edges: vec![],
            faces: vec![],
            attributes,
        }
    }

    pub fn lines(lines: Vec<Edge>, attributes: Attributes) -> MeshResult<Self> {
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

        let mut mesh_edges = Vec::new();
        for Edge { a, b } in lines.iter() {
            let lhs = vertices_map.get(a).ok_or(MeshError::wrong_vert(a))?;
            let rhs = vertices_map.get(b).ok_or(MeshError::wrong_vert(b))?;
            mesh_edges.push(MeshEdge::new(*lhs, *rhs));
        }
        let mut attributes = attributes;
        attributes.set_mesh_type(MeshType::Lines);
        Ok(Mesh {
            vertices,
            edges: mesh_edges,
            faces: vec![],
            attributes,
        })
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

    /// Subdivides the mesh using the specified subdivision algorithm.
    ///
    /// This function returns a `MeshSubdivision` instance that can be used to
    /// apply different subdivision algorithms such as butterfly, loop, or linear.
    ///
    /// # Returns
    ///
    /// A `MeshSubdivision` instance for further subdivision operations.
    pub fn subdivide_by_loop(&self, iterations:usize) -> MeshResult<Mesh> {
        (0..iterations).try_fold(by_loop(self)?, |mesh, _| by_loop(&mesh))
    }

    pub fn contains(&self, v: &Vertex) -> bool {
        self.vertices.contains(v)
    }

    pub fn centroid(&self) -> MeshResult<Vertex> {
        if !self.faces().is_empty() {
            let mut vertices = vec![];
            for face in self.faces() {
                vertices.push(self.face_to_polygon(face)?.centroid()?);
            }
            Polygon::new(vertices).centroid()
        } else {
            Polygon::new(self.vertices().to_vec()).centroid()
        }
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
            .ok_or(MeshError::idx_vertex(idx))
    }
    pub fn triangulate(&self) -> MeshResult<Mesh> {
        let faces = self
            .faces()
            .iter()
            .flat_map(|f| f.triangulate())
            .collect::<Vec<_>>();
        Ok(Mesh::from_vertices(self.vertices().to_vec(), faces, self.attributes().clone()))
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

    pub fn properties(&self) -> properties::MeshProperties {
        properties::MeshProperties::new(self)
    }

    pub fn face_idx_to_polygon(&self, idx: usize) -> MeshResult<Polygon> {
        self.face_to_polygon(self.faces().get(idx).ok_or("Invalid face index")?)
    }
    /// Calculates the distance between the surfaces of two meshes.
    ///
    /// This function computes the minimum distance between the surfaces of the current mesh
    /// and another mesh. It takes into account the vertices, edges, and faces of both meshes
    /// to determine the closest points and the distance between them.
    ///
    /// # Parameters
    ///
    /// * `other` - A reference to the other `Mesh` to calculate the distance to.
    ///
    /// # Returns
    ///
    /// A `MeshResult` containing the distance as an `f32` value if successful, or a `MeshError` if an error occurs.

    pub fn distance(&self, other: &Mesh) -> MeshResult<f32> {
        distance_between_surfaces(self, other)
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
