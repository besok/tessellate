use crate::gpu::error::{GpuError, GpuResult};
use crate::gpu::processor::GpuHandler;
use crate::gpu::vertex::face_to_vertex3;
use log::info;
use std::iter;
use std::sync::Arc;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::window::Window;
use crate::mesh::attributes::MeshType;

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
            let pipeline = &self.pipeline;
            render_pass.set_bind_group(0, &self.camera.camera_bind_group(), &[]);
            for gpu_mesh in self.meshes.iter() {
                let mesh_type = gpu_mesh.mesh.attributes().mesh_type();
                render_pass.set_pipeline(pipeline,);
                match mesh_type {
                    MeshType::Polygons => {
                        render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                        render_pass.draw(0..gpu_mesh.vertices.len() as u32, 0..1);
                    }
                    MeshType::Cloud => {
                        render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                        render_pass.draw(0..gpu_mesh.vertices.len() as u32, 0..1);
                    }
                }

            }
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
                .set_mouse_pressed(*state == ElementState::Pressed),
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
    }
    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }
}
