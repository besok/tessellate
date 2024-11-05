pub mod camera_info;
pub mod controls;

use egui::{Context, FontData, FontDefinitions, FontFamily, Visuals};
use egui_wgpu::Renderer;
use egui_wgpu::ScreenDescriptor;
use std::sync::Arc;

use crate::gpu::error::{GpuError, GpuResult};
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::{CommandEncoder, Queue, StoreOp, TextureView};
use egui_winit::winit::event::WindowEvent;
use egui_winit::winit::window::Window;
use egui_winit::State;
use wgpu::{Device, TextureFormat};

pub struct GuiRenderer {
    frame_started: bool,
    state: State,
    renderer: Renderer,
}

impl GuiRenderer {
    pub fn context(&self) -> &Context {
        self.state.egui_ctx()
    }

    pub fn new(
        device: &Device,
        output_color_format: TextureFormat,
        output_depth_format: Option<TextureFormat>,
        msaa_samples: u32,
        window: Arc<Window>,
    ) -> GpuResult<Self> {
        let egui_context = Context::default();
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("../../assets/fonts/Font_Awesome_6_Free-Solid-900.otf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .ok_or(GpuError::new("No proportional font family"))?
            .insert(0, "my_font".to_owned());
        egui_context.set_fonts(fonts);

        let egui_state = State::new(
            egui_context,
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2 * 1024), // default dimension is 2048
        );
        let egui_renderer =
            Renderer::new(device, output_color_format, output_depth_format, msaa_samples, true);

        Ok(GuiRenderer {
            state: egui_state,
            renderer: egui_renderer,
            frame_started: false,
        })
    }

    pub fn begin_frame(&mut self, window: Arc<Window>) {
        let raw_input = self.state.take_egui_input(window.as_ref());
        self.state.egui_ctx().begin_pass(raw_input);
        self.frame_started = true;
    }

    pub fn set_pixels_per_point(&mut self, v: f32) {
        self.context().set_pixels_per_point(v);
    }
    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _ = self.state.on_window_event(window, event);
    }

    pub fn end_frame_and_draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: Arc<Window>,
        window_surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
    ) -> GpuResult<()> {
        if self.frame_started {
            self.set_pixels_per_point(screen_descriptor.pixels_per_point);

            let full_output = self.state.egui_ctx().end_pass();

            self.state
                .handle_platform_output(window.as_ref(), full_output.platform_output);

            let tris = self
                .state
                .egui_ctx()
                .tessellate(full_output.shapes, self.state.egui_ctx().pixels_per_point());
            for (id, image_delta) in &full_output.textures_delta.set {
                self.renderer
                    .update_texture(device, queue, *id, image_delta);
            }
            self.renderer
                .update_buffers(device, queue, encoder, &tris, &screen_descriptor);
            let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: window_surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                label: Some("egui main render pass"),
                occlusion_query_set: None,
            });

            self.renderer
                .render(&mut rpass.forget_lifetime(), &tris, &screen_descriptor);
            for x in &full_output.textures_delta.free {
                self.renderer.free_texture(x)
            }

            self.frame_started = false;
            Ok(())
        } else {
            return Err(GpuError::new(
                "begin_frame must be called before end_frame_and_draw can be called!",
            ));
        }
    }
}
