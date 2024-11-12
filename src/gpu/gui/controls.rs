
use crate::gpu::camera::Camera;
use egui::{Align2, Color32, Context, RichText};
use glam::Vec3;

pub struct Controls;

impl Controls {
    pub fn show(ctx: &Context, camera: &mut Camera) {
        let speed = camera.camera_coordinator_mut().speed();
        let h_speed = speed / 2.0;
        egui::Window::new("Controls")
            .vscroll(true)
            .default_open(true)
            .auto_sized()
            .anchor(Align2::LEFT_TOP, egui::vec2(1.0, 1.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let camera_coord = camera.camera_coordinator_mut();

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
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let camera_coord = camera.camera_coordinator_mut();

                    let down = ui.button(RichText::new("\u{f063}").size(16.0));
                    if down.clicked() {
                        camera_coord.process_shift_delta(0.0, 0.1);
                    }
                    if down.is_pointer_button_down_on() {
                        camera_coord.process_shift_delta(0.0, h_speed);
                    }

                    let up = ui.button(RichText::new("\u{f062}").size(16.0));
                    if up.clicked() {
                        camera_coord.process_shift_delta(0.0, -0.1);
                    }
                    if up.is_pointer_button_down_on() {
                        camera_coord.process_shift_delta(0.0, -h_speed);
                    }

                    let left = ui.button(RichText::new("\u{f060}").size(16.0));
                    if left.clicked() {
                        camera_coord.process_shift_delta(-0.1, 0.0);
                    }
                    if left.is_pointer_button_down_on() {
                        camera_coord.process_shift_delta(-h_speed, 0.0);
                    }

                    let right = ui.button(RichText::new("\u{f061}").size(16.0));
                    if right.clicked() {
                        camera_coord.process_shift_delta(0.1, 0.0);
                    }
                    if right.is_pointer_button_down_on() {
                        camera_coord.process_shift_delta(h_speed, 0.0);
                    }
                });
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    let pos = camera.camera_pos().position().clone();
                    let rotate_x_plus =
                        ui.button(RichText::new("X+").color(Color32::RED).size(12.0));

                    if rotate_x_plus.clicked() {
                        camera.camera_coordinator_mut().set_x_plus_zero();
                        camera.camera_pos_mut().set_position(Vec3::new(0.0, pos.y, pos.z));
                    }
                    let rotate_x_plus =
                        ui.button(RichText::new("X-").color(Color32::RED).size(12.0));
                    if rotate_x_plus.clicked() {
                        camera.camera_coordinator_mut().set_x_min_zero();
                        camera.camera_pos_mut().set_position(Vec3::new(0.0, pos.y, pos.z));
                    }
                    let rotate_y = ui.button(RichText::new("Y+").color(Color32::GREEN).size(12.0));
                    if rotate_y.clicked() {
                        camera.camera_coordinator_mut().set_y_plus_zero();
                        camera.camera_pos_mut().set_position(Vec3::new(pos.x, 0.0, pos.z));
                    }
                    let rotate_y = ui.button(RichText::new("Y-").color(Color32::GREEN).size(12.0));
                    if rotate_y.clicked() {
                        camera.camera_coordinator_mut().set_y_min_zero();
                        camera.camera_pos_mut().set_position(Vec3::new(pos.x, 0.0, pos.z));
                    }

                    let rotate_z = ui.button(RichText::new("Z+").color(Color32::BLUE).size(12.0));
                    if rotate_z.clicked() {
                        camera.camera_coordinator_mut().set_z_plus_zero();
                        camera.camera_pos_mut().set_position(Vec3::new(pos.x, pos.y, 0.0));
                    }
                    let rotate_z = ui.button(RichText::new("Z-").color(Color32::BLUE).size(12.0));
                    if rotate_z.clicked() {
                        camera.camera_coordinator_mut().set_z_min_zero();
                        camera.camera_pos_mut().set_position(Vec3::new(pos.x, pos.y, 0.0));
                    }

                    let center = ui.button(RichText::new("0").color(Color32::BROWN).size(12.0));
                    if center.clicked() {
                        camera.camera_coordinator_mut().set_angles_0();
                        camera.camera_pos_mut().set_position(Vec3::new(0.0, 0.0, 0.0));
                    }
                    let init_pos = ui.button(RichText::new("\u{f015}").color(Color32::GRAY).size(12.0));
                    if init_pos.clicked() {
                        camera.camera_coordinator_mut().set_init_pos();
                    }
                });
            });
    }
}
