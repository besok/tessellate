use log::LevelFilter;
use crate::gpu::error::GpuError;
use crate::mesh::MeshError;
pub mod gpu;
pub mod mesh;
pub fn turn_on_test_logs() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::max())
        .format_timestamp(None)
        .format_level(false)
        .try_init();
}

pub type TessResult<T> = Result<T, TessError>;

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
