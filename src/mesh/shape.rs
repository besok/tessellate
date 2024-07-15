use crate::mesh::Mesh;

pub mod cuboid;
pub mod sphere;
pub mod icosahedron;
pub mod torus;
pub mod cone;
pub mod cylinder;
pub mod ring;
pub mod pyramid;

pub trait HasMesh {
    fn mesh(self) -> Mesh;
}

impl<T:HasMesh> From<T> for Mesh {
    fn from(value: T) -> Self {
        value.mesh()
    }
}

