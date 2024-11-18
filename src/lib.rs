use crate::files::FileError;
use crate::gpu::error::GpuError;
use crate::mesh::MeshError;
pub mod gpu;
#[macro_use]
pub mod mesh;
pub mod files;


pub type TessResult<T> = Result<T, TessError>;

#[derive(Debug)]
pub enum TessError {
    MeshError(MeshError),
    GpuError(GpuError),
    FileError(FileError),
}



impl From<FileError> for TessError {
    fn from(e: FileError) -> Self {
        TessError::FileError(e)
    }
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
