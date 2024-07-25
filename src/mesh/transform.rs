use crate::mesh::parts::Vertex;
use crate::mesh::{HasMesh, Mesh, MeshResult};
use glam::{Mat4, Vec3};

pub trait Transform {
    fn transform<T: Into<Mat4>>(&mut self, matrix: T) -> MeshResult<()>;
}

impl<HM: HasMesh> Transform for HM {
    fn transform<T: Into<Mat4>>(&mut self, matrix: T) -> MeshResult<()> {
        let _ = &mut self.mesh_mut().transform(matrix);
        Ok(())
    }
}

impl Transform for Mesh {
    fn transform<T: Into<Mat4>>(&mut self, matrix: T) -> MeshResult<()> {
        let tm:Mat4 = matrix.into();
        for vertex in &mut self.vertices {
            vertex.transform(tm)?;
        }
        Ok(())
    }
}

impl Transform for Vertex {
    fn transform<T: Into<Mat4>>(&mut self, matrix: T) -> MeshResult<()> {
        let tm: Mat4 = matrix.into();
        let v: Vec3 = (*self).into();

        let transformed = tm.mul_vec4(v.extend(1.0));
        self.x = transformed.x;
        self.y = transformed.y;
        self.z = transformed.z;

        Ok(())
    }
}
