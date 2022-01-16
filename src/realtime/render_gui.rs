use kintaro_egui_lib::InstanceMul;
use std::{fs::File, io::Write, thread};
use wgpu::TextureView;

use crate::{camera::Camera, clock::Clock, realtime::RealTimeState, save::ConfigState};
use kintaro_egui_lib::{epi::App, ScreenDescriptor};

/// A custom event type for the winit app.
pub enum Event {
    RequestRedraw,
}

pub struct ExampleRepaintSignal(pub std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl kintaro_egui_lib::epi::backend::RepaintSignal for ExampleRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

impl RealTimeState {
    pub fn render_gui(
        &mut self,
        window: &winit::window::Window,
        encoder: &mut wgpu::CommandEncoder,
        view: &TextureView,
    ) {
        self.gui.platform.begin_frame();
        let previous_frame_time = self.clock.current().last_period;
        let app_output = kintaro_egui_lib::epi::backend::AppOutput::default();

        let mut frame =
            kintaro_egui_lib::epi::Frame::new(kintaro_egui_lib::epi::backend::FrameData {
                info: kintaro_egui_lib::epi::IntegrationInfo {
                    name: "egui integration info",
                    web_info: None,
                    cpu_usage: Some(previous_frame_time),
                    native_pixels_per_point: Some(window.scale_factor() as _),
                    prefer_dark_mode: None,
                },
                // tex_allocator: &mut self.gui.renderpass,
                output: app_output,
                repaint_signal: self.repaint_signal.clone(),
            });

        self.gui
            .app
            .update(&self.gui.platform.context(), &mut frame);

        // End the UI frame. We could now handle the output and draw the UI with the backend.
        let (_output, paint_commands) = self.gui.platform.end_frame(Some(window));
        let paint_jobs = self.gui.platform.context().tessellate(paint_commands);

        // Upload all resources for the GPU.
        let screen_descriptor = ScreenDescriptor {
            physical_width: self.size.0,
            physical_height: self.size.1,
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
            .execute(encoder, &view, &paint_jobs, &screen_descriptor, None)
            .unwrap();
    }
}
