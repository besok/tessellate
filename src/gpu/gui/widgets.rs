use crate::gpu::camera::position::CameraPosition;
use egui::{Context, Grid};

pub struct CameraInfo;

impl CameraInfo {
    pub fn show(ctx: &Context, camera_pos: &CameraPosition) {
        egui::Window::new("Camera Info")
            .resizable(true)
            .vscroll(true)
            .default_open(false)
            .show(ctx, |ui| {
                Grid::new("Camera Position")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui|{
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
                    })
            });
    }
}
