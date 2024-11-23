use crate::gpu::camera::position::CameraPosition;
use crate::gpu::error::GpuError;
use crate::gpu::processor::GpuProcessor;
use crate::mesh::Mesh;
use options::GpuOptions;
use winit::event_loop::{ControlFlow, EventLoop};

pub mod camera;
pub mod error;
mod gui;
mod light;
pub mod material;
pub mod options;
mod processor;
mod vertex;

async fn run(meshes: Vec<Mesh>, options: GpuOptions) -> Result<(), GpuError> {
    let camera_pos = CameraPosition::new(options.camera_opts().position(), 0.0, 0.0);
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);
    Ok(event_loop.run_app(&mut GpuProcessor::new(meshes, camera_pos, options))?)
}

/// Visualizes the given meshes using the specified GPU options.
///
/// This function blocks the current thread until the visualization is complete.
///
/// # Arguments
///
/// * `meshes` - A vector of `Mesh` objects to be visualized.
/// * `options` - `GpuOptions` containing the configuration for the GPU.
///
/// # Returns
///
/// * `Result<(), GpuError>` - Returns `Ok(())` if successful, or a `GpuError` if an error occurs.
///
/// # Examples
///
/// ```no_run
/// use tessellate::gpu::options::GpuOptions;
/// use tessellate::gpu::visualize;
/// let meshes = vec![/* ... */];
/// let options = GpuOptions::default();
/// visualize(meshes, options)?;
/// ```
pub fn visualize(meshes: Vec<Mesh>, options: GpuOptions) -> Result<(), GpuError> {
    pollster::block_on(run(meshes, options))
}
