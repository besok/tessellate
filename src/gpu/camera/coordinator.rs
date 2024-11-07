use crate::gpu::camera::position::CameraPosition;
use crate::mesh::parts::bbox::BoundingBox;
use crate::mesh::parts::vertex::Vertex;
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
}

impl CameraCoordinator {
    pub fn new(pos: &Vertex, aabb: BoundingBox, speed: f32, sensitivity: f32) -> Self {
        let c = aabb.center();
        let distance = c.distance(pos);
        let hor_angle = (pos.z - c.z).atan2((pos.x - c.x).hypot(pos.y - c.y));
        let ver_angle = (pos.y - c.y).atan2(pos.x - c.x);
        Self {
            distance,
            hor_angle,
            ver_angle,
            speed,
            sensitivity,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn set_angles_0(&mut self) {
        self.hor_angle = 0.0;
        self.ver_angle = 0.0;
    }
    pub fn set_x_plus_zero(&mut self) {
        self.hor_angle = 0.0;
    }

    pub fn set_x_min_zero(&mut self) {
        self.hor_angle = std::f32::consts::PI;
    }

    pub fn hor_angle(&self) -> f32 {
        self.hor_angle
    }

    pub fn hor_angle_step(&mut self, step: f32) {
        self.hor_angle = self.hor_angle + step;
    }

    pub fn ver_angle(&self) -> f32 {
        self.ver_angle
    }
    pub fn set_y_plus_zero(&mut self) {
        self.hor_angle = std::f32::consts::PI / 2.0;
    }

    pub fn set_y_min_zero(&mut self) {
        self.hor_angle = std::f32::consts::PI / -2.0;
    }

    pub fn set_z_plus_zero(&mut self) {
        self.ver_angle = std::f32::consts::PI / 2.0;
    }

    pub fn set_z_min_zero(&mut self) {
        self.ver_angle = std::f32::consts::PI / -2.0;
    }

    pub fn ver_angle_step(&mut self, step: f32) {
        self.ver_angle = self.ver_angle + step;
    }

    pub fn distance_step(&mut self, step: f32) {
        self.distance = self.distance + step;
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.distance += match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * self.speed,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    pub fn process_mouse(
        &mut self,
        prev_pos: &PhysicalPosition<f64>,
        position: &PhysicalPosition<f64>,
    ) -> bool {
        self.hor_angle += (position.x - prev_pos.x) as f32 * self.sensitivity * self.speed;
        self.ver_angle += (position.y - prev_pos.y) as f32 * self.sensitivity * self.speed;
        true
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
