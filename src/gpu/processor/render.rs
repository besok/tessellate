use crate::gpu::error::{GpuError, GpuResult};
use crate::gpu::gui::camera_info::CameraInfo;
use crate::gpu::gui::controls::Controls;
use crate::gpu::processor::{GpuHandler, Topology};
use crate::gpu::vertex::face_to_vertex3;
use crate::mesh::attributes::MeshType;
use egui::style::Widgets;
use egui_wgpu::wgpu::util::RenderEncoder;
use egui_wgpu::{wgpu, ScreenDescriptor};
use log::info;
use std::iter;
use std::sync::Arc;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::NamedKey::Control;
use winit::window::Window;

impl GpuHandler {
    pub fn render(&mut self) -> GpuResult<()> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            let pipelines = &self.pipelines;
            render_pass.set_bind_group(0, &self.camera.camera_bind_group(), &[]);
            render_pass.set_bind_group(1, &self.light.light_bind_group(), &[]);
            for gpu_mesh in self.meshes.iter() {
                let mesh_type = gpu_mesh.mesh.attributes().mesh_type();
                let pipeline = match mesh_type {
                    MeshType::Polygons | MeshType::Cloud(_) => {
                        pipelines.get(&Topology::TriangleList)
                    }
                    MeshType::Lines => pipelines.get(&Topology::LineList),
                }
                .ok_or(GpuError::General("Pipeline not found".to_string()))?;
                render_pass.set_bind_group(2, &gpu_mesh.material.material_bind_group(), &[]);
                render_pass.set_bind_group(3, &gpu_mesh.affected_by_light, &[]);
                render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                render_pass.set_pipeline(pipeline);
                render_pass.draw(0..gpu_mesh.vertices.len() as u32, 0..1);
            }
        }

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window().scale_factor() as f32,
        };

        {
            self.gui.begin_frame(self.window.clone());
            CameraInfo::show(&self.gui.context(), &self.camera);
            Controls::show(&self.gui.context(), &mut self.camera);

            self.gui.end_frame_and_draw(
                &self.device,
                &self.queue,
                &mut encoder,
                self.window.clone(),
                &view,
                screen_descriptor,
            )?;
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            info!("Resizing to {:?}", new_size);
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.resize(new_size.width, new_size.height);
            self.update()
        }
    }
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            // WindowEvent::KeyboardInput {
            //     event:
            //         KeyEvent {
            //             physical_key: PhysicalKey::Code(key),
            //             state,
            //             ..
            //         },
            //     ..
            // } => self.camera.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => self
                .camera
                .mouse_mut()
                .set_left_button(*state == ElementState::Pressed),
            WindowEvent::MouseInput {
                button: MouseButton::Right,
                state,
                ..
            } => self
                .camera
                .mouse_mut()
                .set_right_button(*state == ElementState::Pressed),

            WindowEvent::CursorMoved { position, .. } if self.camera.is_mouse_pressed() => {
                self.camera.process_mouse(position)
            }
            _ => false,
        }
    }
    pub fn update(&mut self) {
        self.camera.update_camera();
        self.queue.write_buffer(
            &self.camera.camera_buffer(),
            0,
            bytemuck::cast_slice(&[*self.camera.uniform()]),
        );
        self.queue.write_buffer(
            &self.light.light_buffer(),
            0,
            bytemuck::cast_slice(&[*self.light.light_uniform()]),
        );
    }
    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }
}
