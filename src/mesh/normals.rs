use crate::mesh::parts::{Face, Vertex};
use crate::mesh::{Mesh, MeshError, MeshResult};
use nalgebra::Vector3;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct VertexNormals {
    normals: Vec<Vector3<f32>>,
}

impl TryFrom<&Mesh> for VertexNormals {
    type Error = MeshError;

    fn try_from(value: &Mesh) -> Result<Self, Self::Error> {
        VertexNormals::new(value)
    }
}

impl VertexNormals {
    pub fn new(mesh: &Mesh) -> MeshResult<Self> {
        let mut normals: Vec<Vector3<f32>> = vec![Vector3::new(0.0, 0.0, 0.0); mesh.vertices.len()];

        for face in &mesh.faces {
            match face {
                Face::Triangle(v1_idx, v2_idx, v3_idx) => {
                    let v1 = mesh.get_v(*v1_idx)?;
                    let v2 = mesh.get_v(*v2_idx)?;
                    let v3 = mesh.get_v(*v3_idx)?;
                    let face_normal = calculate_triangle_normal(v1, v2, v3);
                    normals[*v1_idx] += face_normal;
                    normals[*v2_idx] += face_normal;
                    normals[*v3_idx] += face_normal;
                }
                Face::Quad(v1_idx, v2_idx, v3_idx, v4_idx) => {
                    let v1 = mesh.get_v(*v1_idx)?;
                    let v2 = mesh.get_v(*v2_idx)?;
                    let v3 = mesh.get_v(*v3_idx)?;
                    let v4 = mesh.get_v(*v4_idx)?;
                    let face_normal = calculate_triangle_normal(v1, v2, v3);
                    normals[*v1_idx] += face_normal;
                    normals[*v2_idx] += face_normal;
                    normals[*v3_idx] += face_normal;
                    normals[*v4_idx] += face_normal;
                }
            }
        }

        Ok(Self {
            normals: normals.into_iter().map(|n| n.normalize()).collect(),
        })
    }

    pub fn get_normal(&self, idx: usize) -> MeshResult<&Vector3<f32>> {
        self.normals
            .get(idx)
            .ok_or(MeshError::InvalidIndex("Invalid vertex index".to_string()))
    }
}

fn calculate_triangle_normal(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vector3<f32> {
    let u = Vector3::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);
    let v = Vector3::new(v3.x - v1.x, v3.y - v1.y, v3.z - v1.z);
    u.cross(&v).normalize()
}
