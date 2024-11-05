use egui::{Align2, Color32, Context, RichText};

use crate::gpu::camera::coordinator::CameraCoordinator;

pub struct Controls;

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
                    let up = ui.button(RichText::new("\u{f358}").size(16.0));
                    if up.clicked() {
                        camera_coord.ver_angle_step(speed);
                    }
                    if up.is_pointer_button_down_on() {
                        camera_coord.ver_angle_step(speed / 2.);
                    }
                    let left = ui.button(RichText::new("\u{f359}").size(16.0));
                    if left.clicked() {
                        camera_coord.hor_angle_step(-speed);
                    }
                    if left.is_pointer_button_down_on() {
                        camera_coord.hor_angle_step(-h_speed);
                    }
                    let right = ui.button(RichText::new("\u{f35a}").size(16.0));
                    if right.clicked() {
                        camera_coord.hor_angle_step(speed);
                    }
                    if right.is_pointer_button_down_on() {
                        camera_coord.hor_angle_step(h_speed);
                    }
                    let down = ui.button(RichText::new("\u{f35b}").size(16.0));
                    if down.clicked() {
                        camera_coord.ver_angle_step(-speed);
                    }
                    if down.is_pointer_button_down_on() {
                        camera_coord.ver_angle_step(-h_speed);
                    }
                    let zoom_in = ui.button(RichText::new("\u{f010}").size(16.0));
                    if zoom_in.clicked() {
                        camera_coord.distance_step(speed);
                    }
                    if zoom_in.is_pointer_button_down_on() {
                        camera_coord.distance_step(h_speed);
                    }
                    let zoom_out = ui.button(RichText::new("\u{f00e}").size(16.0));
                    if zoom_out.clicked() {
                        camera_coord.distance_step(-speed);
                    }
                    if zoom_out.is_pointer_button_down_on() {
                        camera_coord.distance_step(-h_speed);
                    }
                });

                ui.horizontal(|ui| {
                    let rotate_x_plus = ui.button(RichText::new("X+").color(Color32::RED).size(12.0));
                    if rotate_x_plus.clicked() {
                        camera_coord.set_x_plus_zero();
                    }
                    let rotate_x_plus = ui.button(RichText::new("X-").color(Color32::RED).size(12.0));
                    if rotate_x_plus.clicked() {
                        camera_coord.set_x_min_zero();
                    }
                    let rotate_y = ui.button(RichText::new("Y+").color(Color32::GREEN).size(12.0));
                    if rotate_y.clicked() {
                        camera_coord.set_y_plus_zero();
                    }
                    let rotate_y = ui.button(RichText::new("Y-").color(Color32::GREEN).size(12.0));
                    if rotate_y.clicked() {
                        camera_coord.set_y_min_zero();
                    }


                    let rotate_z = ui.button(RichText::new("Z+").color(Color32::BLUE).size(12.0));
                    if rotate_z.clicked() {
                        camera_coord.set_z_plus_zero( );
                    }
                    let rotate_z = ui.button(RichText::new("Z-").color(Color32::BLUE).size(12.0));
                    if rotate_z.clicked() {
                        camera_coord.set_z_min_zero( );
                    }
                });
            });
    }
}
