use crate::gpu::camera::position::CameraPosition;
use glam::Vec3;
use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

#[derive(Debug)]
pub struct CameraCoordinator {
    distance: f32,
    hor_angle: f32,
    ver_angle: f32,
    speed: f32,
    sensitivity: f32,
    last_mouse_pos: Option<PhysicalPosition<f64>>,
}

impl CameraCoordinator {
    pub fn new(distance: f32, speed: f32, sensitivity: f32) -> Self {
        Self {
            distance,
            hor_angle: 0.0,
            ver_angle: 0.0,
            speed,
            sensitivity,
            last_mouse_pos: None,
        }
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.distance += match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * self.speed,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    pub fn process_mouse(&mut self, position: &PhysicalPosition<f64>) -> bool {
        if let Some(last_pos) = self.last_mouse_pos {
            self.hor_angle += (position.x - last_pos.x) as f32 * self.sensitivity * self.speed;
            self.ver_angle += (position.y - last_pos.y) as f32 * self.sensitivity * self.speed;
        } else {
            self.last_mouse_pos = Some(*position);
        }

        true
    }

    pub fn clean_mouse_pos(&mut self) {
        self.last_mouse_pos = None;
    }

    pub fn update_camera(&mut self, camera: &mut CameraPosition) {
        // Apply circular movement

        let new_x = self.distance * self.hor_angle.cos();
        let new_z = self.distance * self.hor_angle.sin();
        let new_y = self.distance * self.ver_angle.sin();
        let new_position = Vec3::new(new_x, new_y, new_z);
        camera.set_position(new_position);

        let direction = (Vec3::ZERO - new_position).normalize();
        camera.set_yaw(direction.z.atan2(direction.x));
        camera.set_pitch(direction.y.asin());
    }
}
