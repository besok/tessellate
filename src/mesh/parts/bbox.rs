use crate::mesh::attributes::Attributes;
use crate::mesh::material::Color;
use crate::mesh::parts::face::FaceType;
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::shape::cuboid::rect_cuboid::RectCuboid;
use crate::mesh::Mesh;

/// Bounding box
/// The structure to store the bounding box of a mesh
/// The bounding box is used to determine the intersection of the mesh
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

    /// Create a bounding box from a polygon
    /// The bounding box is created from the minimum and maximum vertices of the polygon
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

    /// Check if the bounding box intersects with a polygon
    /// The function checks if any of the vertices of the polygon is within the bounding box
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

    /// Merge two bounding boxes
    /// The function merges two bounding boxes into a single bounding box
    /// The new bounding box has the minimum vertex of the two bounding boxes as the minimum vertex
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

    /// The function creates a mesh from the bounding box
    pub fn to_rect_cuboid<C>(self, face_type: FaceType, attributes: C) -> RectCuboid
    where
        C: Into<Attributes>,
    {
        RectCuboid::create_bbox(self.min_vertex, self.max_vertex, face_type, attributes)
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

    /// The function returns the center of the bounding box
    pub(crate) fn center(&self) -> Vertex {
        Vertex::new(
            (self.min_vertex.x + self.max_vertex.x) / 2.0,
            (self.min_vertex.y + self.max_vertex.y) / 2.0,
            (self.min_vertex.z + self.max_vertex.z) / 2.0,
        )
    }

    /// The function checks if the bounding box intersects with another bounding box
    /// The function checks if the minimum and maximum vertices of the two bounding boxes overlap
    /// If the minimum and maximum vertices overlap, the bounding boxes intersect
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let x_overlap =
            self.min_vertex.x <= other.max_vertex.x && self.max_vertex.x >= other.min_vertex.x;
        let y_overlap =
            self.min_vertex.y <= other.max_vertex.y && self.max_vertex.y >= other.min_vertex.y;
        let z_overlap =
            self.min_vertex.z <= other.max_vertex.z && self.max_vertex.z >= other.min_vertex.z;

        x_overlap && y_overlap && z_overlap
    }
    /// The function checks if the bounding box contains a vertex
    pub fn contains(&self, vertex: &Vertex) -> bool {
        let x_within = self.min_vertex.x <= vertex.x && vertex.x <= self.max_vertex.x;
        let y_within = self.min_vertex.y <= vertex.y && vertex.y <= self.max_vertex.y;
        let z_within = self.min_vertex.z <= vertex.z && vertex.z <= self.max_vertex.z;

        x_within && y_within && z_within
    }

    /// Calculate the distance from the center of the bounding box to a given vertex
    ///
    /// # Arguments
    ///
    /// * `v` - A reference to the vertex to which the distance is calculated
    ///
    /// # Returns
    ///
    /// * `f32` - The distance from the center of the bounding box to the given vertex
    pub fn distance(&self, v: &Vertex) -> f32 {
        self.center().distance(v)
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
