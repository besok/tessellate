use crate::mesh::Mesh;

pub mod cuboid;
pub mod sphere;
pub mod icosahedron;

pub trait HasMesh {
    fn mesh(self) -> Mesh;
}

impl<T:HasMesh> From<T> for Mesh {
    fn from(value: T) -> Self {
        value.mesh()
    }
}

