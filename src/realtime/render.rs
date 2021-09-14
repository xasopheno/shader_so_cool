use crate::{
    clock::Clock,
    realtime::RealTimeState,
    shared::{render_pass, update},
};
use chrono::Timelike;
use egui_wgpu_backend::ScreenDescriptor;
use epi::*;
/// Time of day as seconds since midnight. Used for clock in demo app.
pub fn seconds_since_midnight() -> f64 {
    let time = chrono::Local::now().time();
    time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}

/// A custom event type for the winit app.
pub enum Event {
    RequestRedraw,
}

pub struct ExampleRepaintSignal(pub std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl epi::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        self.clock.update();
        let time = self.clock.current();
        self.camera.update(time.last_period);

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (&self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        for renderpass in self.renderpasses.iter_mut() {
            update(
                time,
                renderpass,
                &self.device,
                &self.queue,
                (self.size.width, self.size.height),
                &self.canvas,
            );
        }

        let output = self.surface.get_current_frame()?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        for (n, renderpass) in self.renderpasses.iter_mut().enumerate() {
            renderpass
                .uniforms
                .update_view_proj(view_position, view_proj);
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            let accumulation = if n == 0 { false } else { true };

            render_pass(&mut encoder, &renderpass, &view, &self.config, accumulation);

            self.queue.submit(std::iter::once(encoder.finish()));
        }

        self.gui.platform.begin_frame();
        let previous_frame_time = time.last_period;
        let mut app_output = epi::backend::AppOutput::default();

        let mut frame = epi::backend::FrameBuilder {
            info: epi::IntegrationInfo {
                web_info: None,
                cpu_usage: Some(previous_frame_time),
                seconds_since_midnight: Some(seconds_since_midnight()),
                native_pixels_per_point: Some(window.scale_factor() as _),
                prefer_dark_mode: None,
            },
            tex_allocator: &mut self.gui.renderpass,
            output: &mut app_output,
            repaint_signal: self.repaint_signal.clone(),
        }
        .build();
        self.gui
            .app
            .update(&self.gui.platform.context(), &mut frame);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self.gui.platform.end_frame(Some(window));
        let paint_jobs = self.gui.platform.context().tessellate(paint_commands);
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: self.size.width,
            physical_height: self.size.height,
            scale_factor: window.scale_factor() as f32,
        };
        self.gui.renderpass.update_texture(
            &self.device,
            &self.queue,
            &self.gui.platform.context().texture(),
        );
        self.gui
            .renderpass
            .update_user_textures(&self.device, &self.queue);

        self.gui.renderpass.update_buffers(
            &mut self.device,
            &mut self.queue,
            &paint_jobs,
            &screen_descriptor,
        );

        // Record all render passes.
        self.gui
            .renderpass
            .execute(
                &mut encoder,
                &view,
                &paint_jobs,
                &screen_descriptor,
                None,
                // Some(wgpu::Color::TRANSPARENT),
            )
            .unwrap();

        // // Submit the commands.
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
