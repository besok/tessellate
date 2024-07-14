use std::error::Error;
use std::fmt::{Display, Formatter};
use wgpu::SurfaceError;
use crate::mesh::MeshError;

#[derive(Debug, Clone)]
pub enum GpuError {
    General(String),
    WgpuSurfaceError(SurfaceError),
}

impl GpuError {
    pub fn new(message: &str) -> Self {
        GpuError::General(message.to_string())
    }
}

impl From<SurfaceError> for GpuError {
    fn from(e: SurfaceError) -> Self {
        GpuError::WgpuSurfaceError(e)
    }
}

impl From<MeshError> for GpuError {
    fn from(e: MeshError) -> Self {
        GpuError::General(format!("Mesh error: {:?}", e))
    }
}

impl From<winit::error::OsError> for GpuError {
    fn from(e: winit::error::OsError) -> Self {
        GpuError::General(format!("windows error: {}", e))
    }
}

impl From<wgpu::CreateSurfaceError> for GpuError {
    fn from(e: wgpu::CreateSurfaceError) -> Self {
        GpuError::General(format!("Create surface error: {}", e))
    }
}

impl From<wgpu::RequestDeviceError> for GpuError {
    fn from(e: wgpu::RequestDeviceError) -> Self {
        GpuError::General(format!("Request device error: {}", e))
    }
}

pub type GpuResult<T> = Result<T, GpuError>;

impl Display for GpuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GpuError::General(message) => write!(f, "{}", message),
            GpuError::WgpuSurfaceError(e) => write!(f, "Wgpu surface error: {}", e),
        }
    }
}

impl Error for GpuError {}
