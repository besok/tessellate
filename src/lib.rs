use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};
use winit::window::Window;

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let window_attributes = Window::default_attributes().with_title("A fantastic window!");
    let window = event_loop.create_window(window_attributes).unwrap();

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                event:
                KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                    ..
                },
                ..
            } => control_flow.exit(),
            _ => {}
        },
        _ => {}
    });
}
