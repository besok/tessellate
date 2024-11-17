use crate::mesh::parts::bbox::BoundingBox;
use crate::mesh::parts::vertex::Vertex;

use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;

#[derive(Debug)]
pub struct CameraCoordinator {
    distance: f32,
    target: Vertex,
    init_target: Vertex,
    init_eye: Vertex,
    eye: Vertex,
    hor_angle: f32,
    ver_angle: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraCoordinator {
    pub fn new(pos: &Vertex, aabb: BoundingBox, speed: f32, sensitivity: f32) -> Self {
        let target = aabb.center();
        let init_target = aabb.center();
        let distance = target.distance(pos);
        let eye = pos.clone();
        let init_eye = pos.clone();
        let (hor_angle, ver_angle) = angles(&eye, &target);
        Self {
            init_target,
            init_eye,
            distance,
            eye,
            target,
            hor_angle,
            ver_angle,
            speed: speed * 0.1,
            sensitivity: sensitivity * 0.1,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn eye(&self) -> Vertex {
        self.eye
    }

    pub fn target(&self) -> Vertex {
        self.target
    }

    pub fn set_angles_0(&mut self) {
        self.hor_angle = 0.0;
        self.ver_angle = 0.0;
        self.target = self.init_target.clone();
        self.derive_new_source();
    }

    pub fn set_init_pos(&mut self) {
        self.target = self.init_target.clone();
        self.eye = self.init_eye.clone();
        self.distance = self.target.distance(&self.eye);
        let (ha, va) = angles(&self.eye, &self.target);
        self.hor_angle = ha;
        self.ver_angle = va;
    }

    pub fn set_x_plus_zero(&mut self) {
        self.hor_angle = 0.0;
        self.derive_new_source();
    }

    pub fn set_x_min_zero(&mut self) {
        self.hor_angle = std::f32::consts::PI;
        self.derive_new_source();
    }

    pub fn hor_angle(&self) -> f32 {
        self.hor_angle
    }

    pub fn hor_angle_step(&mut self, step: f32) {
        self.hor_angle = self.hor_angle + step;
        self.derive_new_source();
    }

    pub fn ver_angle(&self) -> f32 {
        self.ver_angle
    }
    pub fn set_y_plus_zero(&mut self) {
        self.hor_angle = std::f32::consts::PI / 2.0;
        self.derive_new_source();
    }

    pub fn set_y_min_zero(&mut self) {
        self.hor_angle = std::f32::consts::PI / -2.0;
        self.derive_new_source();
    }

    pub fn set_z_plus_zero(&mut self) {
        self.ver_angle = std::f32::consts::PI / 2.0;
        self.derive_new_source();
    }

    pub fn set_z_min_zero(&mut self) {
        self.ver_angle = std::f32::consts::PI / -2.0;
        self.derive_new_source();
    }

    pub fn ver_angle_step(&mut self, step: f32) {
        self.ver_angle = self.ver_angle + step;
        self.derive_new_source();
    }

    pub fn distance_step(&mut self, step: f32) {
        self.distance = self.distance + step;
        self.derive_new_source();
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
        self.derive_new_source();
    }

    fn derive_new_source(&mut self) {
        let x = self.target.x + self.distance * self.hor_angle.cos() * self.ver_angle.cos();
        let y = self.target.y + self.distance * self.ver_angle.sin();
        let z = self.target.z + self.distance * self.hor_angle.sin() * self.ver_angle.cos();
        self.eye = Vertex::new(x, y, z)
    }

    pub fn process_rot(
        &mut self,
        prev_pos: &PhysicalPosition<f64>,
        position: &PhysicalPosition<f64>,
    ) -> bool {
        self.hor_angle += (position.x - prev_pos.x) as f32 * self.sensitivity * self.speed;
        self.ver_angle += (position.y - prev_pos.y) as f32 * self.sensitivity * self.speed;
        self.derive_new_source();
        true
    }
    pub fn process_shift(
        &mut self,
        curr_pos: &PhysicalPosition<f64>,
        new_pos: &PhysicalPosition<f64>,
    ) -> bool {
        let dx = (new_pos.x - curr_pos.x) as f32 * self.speed * self.sensitivity;
        let dy = (new_pos.y - curr_pos.y) as f32 * self.speed * self.sensitivity;

        self.process_shift_delta(dx, dy)
    }

    pub fn process_shift_delta(&mut self, dx: f32, dy: f32) -> bool {
        let normal = (self.eye - self.target).normalize();
        let up = if normal.x.abs() < 1e-6 && normal.z.abs() < 1e-6 {
            Vertex::new(1.0, 0.0, 0.0)
        } else {
            Vertex::new(0.0, 1.0, 0.0)
        };
        let right = normal.cross(&up).normalize();
        let up = right.cross(&normal).normalize();

        let delta: Vertex = (right * dx + up * dy).into();
        self.eye = self.eye + delta;
        self.target = self.target + delta;

        true
    }
}

type HAngle = f32;
type VAngle = f32;

fn angles(src: &Vertex, trg: &Vertex) -> (HAngle, VAngle) {
    let hor_angle = (src.z - trg.z).atan2((src.x - trg.x).hypot(src.y - trg.y));
    let ver_angle = (src.y - trg.y).atan2(src.x - trg.x);
    (hor_angle, ver_angle)
}
