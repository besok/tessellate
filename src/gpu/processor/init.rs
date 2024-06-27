use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use crate::gpu::GpuException;
use crate::gpu::processor::State;

pub(crate) fn init_state(event_loop: &ActiveEventLoop) -> Result<State, GpuException> {
    let window = event_loop.create_window(Window::default_attributes())?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        #[cfg(not(target_arch = "wasm32"))]
        backends: wgpu::Backends::PRIMARY,
        #[cfg(target_arch = "wasm32")]
        backends: wgpu::Backends::GL,
        ..Default::default()
    });
    let surface = instance.create_surface(&window)?;

    let adapter = pollster::block_on(instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).ok_or(GpuException("no adapters found".to_string()))?;



    Ok(State::init(&window))
}