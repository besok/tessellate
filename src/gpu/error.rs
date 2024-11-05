use crate::mesh::MeshError;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::SurfaceError;
use image::ImageError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use winit::error::EventLoopError;
use winit::window::BadIcon;

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

impl From<io::Error> for GpuError {
    fn from(e: io::Error) -> Self {
        GpuError::General(format!("IO error: {}", e))
    }
}


impl From<BadIcon> for GpuError {
    fn from(e: BadIcon) -> Self {
        GpuError::General(format!("Bad icon error: {}", e))
    }
}
impl From<ImageError> for GpuError {
    fn from(e: ImageError) -> Self {
        GpuError::General(format!("Image error: {}", e))
    }
}
