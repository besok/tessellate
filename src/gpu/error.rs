use wgpu::SurfaceError;
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GpuError(String);

impl GpuError {
    pub fn new(message: &str) -> Self {
        GpuError(message.to_string())
    }
}

impl From<SurfaceError> for GpuError {
    fn from(e: SurfaceError) -> Self {
        GpuError(format!("Surface error: {}", e))
    }
}

impl From< winit::error::OsError> for GpuError {
    fn from(e: winit::error::OsError) -> Self {
        GpuError(format!("windows error: {}", e))
    }
}

impl From<wgpu::CreateSurfaceError> for GpuError {
    fn from(e: wgpu::CreateSurfaceError) -> Self {
        GpuError(format!("Create surface error: {}", e))
    }
}

impl From<wgpu::RequestDeviceError> for GpuError {
    fn from(e: wgpu::RequestDeviceError) -> Self {
        GpuError(format!("Request device error: {}", e))
    }
}

pub type GpuResult<T> = Result<T, GpuError>;

impl Display for GpuError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for GpuError {}