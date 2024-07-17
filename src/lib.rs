use crate::gpu::error::GpuError;
use crate::mesh::MeshError;

pub mod gpu;
pub mod mesh;


#[derive(Debug)]
pub enum TessError {
    MeshError(MeshError),
    GpuError(GpuError),
}

impl From<MeshError> for TessError {
    fn from(e: MeshError) -> Self {
        TessError::MeshError(e)
    }
}

impl From<GpuError> for TessError {
    fn from(e: GpuError) -> Self {
        TessError::GpuError(e)
    }
}
