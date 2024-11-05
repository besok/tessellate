use egui::{Align2, Context, Grid};
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
