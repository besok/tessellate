use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshError, MeshResult};
use glam::Vec3;
use std::collections::HashMap;
use crate::mesh::parts::vertex::Vertex;

/// Mesh normals
/// The structure to store normals for vertices and faces
/// The normals are calculated based on the faces
/// The normals for vertices are calculated
/// as the sum of the normals of the faces that share the vertex
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MeshNormals {
    normals_vert: Vec<Vec3>,
    normals_face: Vec<Vec3>,
    normals_face_map: HashMap<Face, Vec3>,
}

impl TryFrom<&Mesh> for MeshNormals {
    type Error = MeshError;

    fn try_from(value: &Mesh) -> MeshResult<Self> {
        MeshNormals::new(value)
    }
}

pub fn calculate_normal(vertices: &Vec<Vertex>) -> Vec3 {
    if vertices.len() < 3 {
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        let v0 = &vertices[0];
        let v1 = &vertices[1];
        let v2 = &vertices[2];
        calculate_triangle_normal(v0, v1, v2)
    }
}

impl MeshNormals {
    pub fn new(mesh: &Mesh) -> MeshResult<Self> {
        let mut normals: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); mesh.vertices.len()];
        let mut face_normals: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0); mesh.faces.len()];
        let mut face_normals_map: HashMap<Face, Vec3> = HashMap::new();

        for face in &mesh.faces {
            match face {
                Face::Triangle(v1_idx, v2_idx, v3_idx) => {
                    let v1 = mesh.get(*v1_idx)?;
                    let v2 = mesh.get(*v2_idx)?;
                    let v3 = mesh.get(*v3_idx)?;
                    let face_normal = calculate_triangle_normal(v1, v2, v3);
                    face_normals.push(face_normal);
                    face_normals_map.insert(face.clone(), face_normal);
                    normals[*v1_idx] += face_normal;
                    normals[*v2_idx] += face_normal;
                    normals[*v3_idx] += face_normal;
                }
                Face::Quad(v1_idx, v2_idx, v3_idx, v4_idx) => {
                    let v1 = mesh.get(*v1_idx)?;
                    let v2 = mesh.get(*v2_idx)?;
                    let v3 = mesh.get(*v3_idx)?;
                    let _v4 = mesh.get(*v4_idx)?;
                    let face_normal = calculate_triangle_normal(v1, v2, v3);
                    face_normals_map.insert(face.clone(), face_normal);
                    face_normals.push(face_normal);
                    normals[*v1_idx] += face_normal;
                    normals[*v2_idx] += face_normal;
                    normals[*v3_idx] += face_normal;
                    normals[*v4_idx] += face_normal;
                }
            }
        }

        Ok(Self {
            normals_vert: normals.into_iter().map(|n| n.normalize()).collect(),
            normals_face: face_normals.into_iter().map(|n| n.normalize()).collect(),
            normals_face_map: face_normals_map,
        })
    }

    /// Get the normal for a vertex
    pub fn get_normal(&self, idx: usize) -> MeshResult<&Vec3> {
        self.normals_vert
            .get(idx)
            .ok_or(MeshError::InvalidIndex("Invalid vertex index".to_string()))
    }
}

fn calculate_triangle_normal(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec3 {
    let u:Vec3 = (*v2 - *v1).into();
    let v:Vec3 = (*v3 - *v1).into();
    u.cross(v).normalize()
}
