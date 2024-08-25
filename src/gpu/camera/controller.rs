use glam::Vec3;
use crate::gpu::camera::position::CameraPosition;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseScrollDelta};
use winit::keyboard::KeyCode;
use crate::gpu::camera::projection::SAFE_FRAC_PI_2;

#[derive(Debug)]
pub struct CameraController {
    left: f32,
    right: f32,
    forward: f32,
    backward: f32,
    up: f32,
    down: f32,
    rotate_hor: f32,
    rotate_ver: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    radius: f32,
    hor_angle: f32,
    ver_angle: f32,
    last_mouse_pos: PhysicalPosition<f64>,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            forward: 0.0,
            backward: 0.0,
            up: 0.0,
            down: 0.0,
            rotate_hor: 0.0,
            rotate_ver: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            radius: 5.0,
            hor_angle: 0.0,
            ver_angle: 0.0,
            last_mouse_pos: PhysicalPosition::new(0.0, 0.0),
        }
    }

    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        match key {

            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.forward = amount;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.backward = amount;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.left = amount;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.right = amount;
                true
            }
            KeyCode::Space => {
                self.up = amount;
                true
            }
            KeyCode::ShiftLeft => {
                self.down = amount;
                true
            }
            KeyCode::KeyR => {
                self.radius += amount * self.speed;
                true
            }
            KeyCode::KeyF => {
                self.radius -= amount * self.speed;
                true
            }
            KeyCode::KeyT => {
                self.hor_angle += amount * self.sensitivity;
                true
            }
            KeyCode::KeyG => {
                self.hor_angle -= amount * self.sensitivity;
                true
            }
            KeyCode::KeyY => {
                self.ver_angle += amount * self.sensitivity;
                true
            }
            KeyCode::KeyH => {
                self.ver_angle -= amount * self.sensitivity;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, position:&PhysicalPosition<f64>) -> bool {

        let mouse_dx = position.x - self.last_mouse_pos.x;
        let mouse_dy = position.y - self.last_mouse_pos.y;
        self.last_mouse_pos = *position;

        self.rotate_hor = mouse_dx as f32;
        self.rotate_ver = mouse_dy as f32;

        true
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => *scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut CameraPosition) {

        // Apply circular movement

        println!("radius: {}", self.radius);
        println!("hor_angle: {}", self.hor_angle);
        println!("ver_angle: {}", self.ver_angle);

        println!("camera position: {:?}", camera.position());

        let new_x = self.radius * self.hor_angle.cos();
        let new_z = self.radius * self.hor_angle.sin();
        let new_y = self.radius * self.ver_angle.sin();
        let new_position = Vec3::new(new_x, new_y, new_z);
        camera.set_position(new_position);

        println!("new position: {:?}", new_position);

        // Calculate the new direction vector
        let direction = (Vec3::ZERO - new_position).normalize();
        camera.set_yaw(direction.z.atan2(direction.x));
        camera.set_pitch(direction.y.asin());

        println!("new yaw: {}", direction.z.atan2(direction.x));
        println!("new pitch: {}", direction.y.asin());

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw().sin_cos();
        let forward = Vec3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.update_position(forward * (self.forward - self.backward) * self.speed );
        camera.update_position(right * (self.right - self.left) * self.speed );


        let (pitch_sin, pitch_cos) = camera.pitch().sin_cos();
        let scrollward =
            Vec3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.update_position(scrollward * self.scroll * self.speed * self.sensitivity);
        self.scroll = 0.0;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.shift_y((self.up - self.down) * self.speed );

        // Rotate
        camera.update_yaw(self.rotate_hor * self.sensitivity );
        camera.update_pitch(-self.rotate_ver * self.sensitivity);




        // // If process_mouse isn't called every frame, these values
        // // will not get set to zero, and the camera will rotate
        // // when moving in a non-cardinal direction.
        self.rotate_hor = 0.0;
        self.rotate_ver = 0.0;

        if camera.pitch() < -SAFE_FRAC_PI_2 {
            camera.set_pitch(-SAFE_FRAC_PI_2);
        } else if camera.pitch() > SAFE_FRAC_PI_2 {
            camera.set_pitch(SAFE_FRAC_PI_2);
        }
    }
}
