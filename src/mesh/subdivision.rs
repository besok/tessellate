use crate::mesh::{Mesh, MeshResult};

pub struct MeshSubdivision<'a> {
    mesh: &'a Mesh,
}

impl<'a> MeshSubdivision<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        MeshSubdivision { mesh }
    }

    pub fn by_butterfly(&self) -> MeshResult<Mesh> {
        Ok(self.mesh.clone())
    }

    pub fn by_loop(&self) -> MeshResult<Mesh> {
        Ok(self.mesh.clone())
    }

    pub fn by_linear(&self) -> MeshResult<Mesh> {
        Ok(self.mesh.clone())
    }
}