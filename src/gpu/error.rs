use crate::mesh::MeshError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use wgpu::SurfaceError;
use winit::error::EventLoopError;

#[derive(Debug, Clone)]
pub enum GpuError {
    General(String),
    WgpuSurfaceError(SurfaceError),
    EventLoopError(String),
    MeshError(MeshError),
}

impl GpuError {
    pub fn new(message: &str) -> Self {
        GpuError::General(message.to_string())
    }
}

impl From<MeshError> for GpuError {
    fn from(e: MeshError) -> Self {
        GpuError::MeshError(e)
    }
}

impl From<EventLoopError> for GpuError {
    fn from(e: EventLoopError) -> Self {
        GpuError::EventLoopError(format!("Event loop error: {}", e))
    }
}

impl From<SurfaceError> for GpuError {
    fn from(e: SurfaceError) -> Self {
        GpuError::WgpuSurfaceError(e)
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
            GpuError::EventLoopError(_) => write!(f, "Event loop error"),
            GpuError::MeshError(_) => write!(f, "Mesh error"),
        }
    }
}

impl Error for GpuError {}
