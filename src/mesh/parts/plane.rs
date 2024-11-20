use glam::Vec3;
use crate::mesh::{Mesh, MeshError, MeshResult};
use crate::mesh::attributes::Attributes;
use crate::mesh::material::Color;
use crate::mesh::normals::calculate_normal;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    normal: Vec3,
    point: Vec3,
    dist: f32,
}

impl Plane {
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn from_polygon(polygon: &Polygon) -> MeshResult<Self> {
        create_plane(polygon)
    }
    pub fn new(normal: Vec3, point: Vec3) -> Self {
        let dist = normal.dot(point);
        Self {
            normal,
            point,
            dist,
        }
    }

    pub fn distance<T: Into<Vec3>>(&self, point: T) -> f32 {
        let point = point.into();
        self.normal.dot(point) - self.dist
    }

    pub fn to_mesh(&self, size: f32, attrs: Attributes) -> Mesh {
        let right = self.normal.cross(Vec3::Y).normalize() * size;
        let up = self.normal.cross(right).normalize() * size;

        let vertices: Vec<Vertex> = vec![
            (self.point - right - up).into(),
            (self.point + right - up).into(),
            (self.point + right + up).into(),
            (self.point - right + up).into(),
        ];

        let faces = vec![Face::Triangle(0, 1, 2), Face::Triangle(2, 3, 0)];

        Mesh::from_vertices(vertices, faces, attrs )
    }
}

fn create_plane(polygon: &Polygon) -> MeshResult<Plane> {
    let normal = calculate_normal(polygon.vertices());
    let point = polygon
        .vertices()
        .first()
        .ok_or(MeshError::InvalidIndex("No vertices".into()))?;
    Ok(Plane::new(normal.clone(), point.into()))
}