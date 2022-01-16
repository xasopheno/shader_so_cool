use crate::{camera::Camera, clock::Clock, realtime::RealTimeState, save::ConfigState};
use kintaro_egui_lib::{epi::App, ScreenDescriptor};
use std::{fs::File, io::Write, thread};
use wgpu::TextureView;

/// A custom event type for the winit app.
pub enum Event {
    RequestRedraw,
}

pub struct GuiRepaintSignal(pub std::sync::Mutex<winit::event_loop::EventLoopProxy<Event>>);

impl kintaro_egui_lib::epi::backend::RepaintSignal for GuiRepaintSignal {
    fn request_repaint(&self) {
        self.0.lock().unwrap().send_event(Event::RequestRedraw).ok();
    }
}

impl RealTimeState {
    pub fn handle_save(&mut self) {
        {
            let mut state = self.gui.state.lock().unwrap();
            if state.save {
                let filename = "./save/saved.json";
                let instance_mul = state.instance_mul.to_owned();
                let camera = self.composition.camera.current_state().clone();
                thread::spawn(move || {
                    let mut file = File::create(filename).unwrap();
                    let config_state = ConfigState {
                        camera,
                        instance_mul,
                    };
                    let serialized = serde_json::to_string(&config_state)
                        .expect(&format!("unable to serialize, {}", filename));
                    file.write(serialized.as_bytes())
                        .expect("unable to write to file on save");
                });
                state.save = false;
                println!("Saved {}", filename);
            }
        }
    }

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
            &self.gui.platform.context().font_image(),
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

    pub fn update_gui(&mut self) {
        let s = self.gui.state.lock().unwrap();
        self.audio_stream_handle.set_volume(s.volume);
        if s.camera_index != self.composition.camera.index {
            self.composition.camera = Camera::new(
                &self.composition.config.cameras[s.camera_index],
                self.composition.config.window_size,
                &self.composition.config,
                s.camera_index,
            )
        }
        if !s.play && !self.audio_stream_handle.is_paused() {
            self.audio_stream_handle.pause();
        };
        if s.play && self.audio_stream_handle.is_paused() {
            self.audio_stream_handle.play();
        };
        self.clock.set_playing(s.play);
        self.audio_stream_handle.set_volume(s.volume);
    }
}