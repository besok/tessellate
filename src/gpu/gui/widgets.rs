use egui::{Align2, Color32, Context, Grid, RichText, TextureHandle};

use crate::gpu::camera::coordinator::CameraCoordinator;
use crate::gpu::camera::Camera;

pub struct CameraInfo;

impl CameraInfo {
    pub fn show(ctx: &Context, camera: &Camera) {
        let camera_pos = camera.camera_pos();
        let camera_coord = camera.camera_coordinator();
        egui::Window::new("Camera Info")
            .anchor(Align2::RIGHT_TOP, egui::vec2(1.0, 1.0))
            .vscroll(true)
            .default_open(false)
            .auto_sized()
            .show(ctx, |ui| {
                Grid::new("Camera Position")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Position");
                        ui.label(format!("X: {:.1}", camera_pos.position().x));
                        ui.label(format!("Y: {:.1}", camera_pos.position().y));
                        ui.label(format!("Z: {:.1}", camera_pos.position().z));
                        ui.end_row();

                        ui.label("Rotation Y(Yaw)");
                        ui.label(format!(": {:.0}", camera_pos.yaw().to_degrees()));
                        ui.end_row();

                        ui.label("Rotation X(Pitch)");
                        ui.label(format!(": {:.0}", camera_pos.pitch().to_degrees()));
                        ui.end_row();

                        ui.label("Distance");
                        ui.label(format!(": {:.1}", camera_coord.distance()));
                        ui.end_row();

                        ui.label("Speed");
                        ui.label(format!(": {:.1}", camera_coord.speed()));
                        ui.end_row();

                        ui.label("Sensitivity");
                        ui.label(format!(": {:.1}", camera_coord.sensitivity()));
                        ui.end_row();
                    })
            });
    }
}

pub struct Controls {}

impl Controls {

    pub fn show(ctx: &Context, camera_coord: &mut CameraCoordinator) {
        let speed = camera_coord.speed();
        let h_speed = speed / 2.0;
        egui::Window::new("Controls")
            .vscroll(true)
            .default_open(true)
            .auto_sized()
            .anchor(Align2::LEFT_TOP, egui::vec2(1.0, 1.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let up = ui.button(RichText::new("↑").color(Color32::GREEN).size(16.0));
                    if up.clicked() {
                        camera_coord.ver_angle_step(speed);
                    }
                    if up.is_pointer_button_down_on() {
                        camera_coord.ver_angle_step(speed / 2.);
                    }
                    let left = ui.button(RichText::new("←").color(Color32::GREEN).size(16.0));
                    if left.clicked() {
                        camera_coord.hor_angle_step(-speed);
                    }
                    if left.is_pointer_button_down_on() {
                        camera_coord.hor_angle_step(-h_speed);
                    }
                    let right = ui.button(RichText::new("→").color(Color32::GREEN).size(16.0));
                    if right.clicked() {
                        camera_coord.hor_angle_step(speed);
                    }
                    if right.is_pointer_button_down_on() {
                        camera_coord.hor_angle_step(h_speed);
                    }
                    let down = ui.button(RichText::new("↓").color(Color32::GREEN).size(16.0));
                    if down.clicked() {
                        camera_coord.ver_angle_step(-speed);
                    }
                    if down.is_pointer_button_down_on() {
                        camera_coord.ver_angle_step(-h_speed);
                    }
                    let zoom_in = ui.button(RichText::new("+").color(Color32::GREEN).size(16.0));
                    if zoom_in.clicked() {
                        camera_coord.distance_step(speed);
                    }
                    if zoom_in.is_pointer_button_down_on() {
                        camera_coord.distance_step(h_speed);
                    }
                    let zoom_out = ui.button(RichText::new("-").color(Color32::GREEN).size(16.0));
                    if zoom_out.clicked() {
                        camera_coord.distance_step(-speed);
                    }
                    if zoom_out.is_pointer_button_down_on() {
                        camera_coord.distance_step(-h_speed);
                    }
                });
            });
    }
}
