use crate::{camera::Camera, clock::Clock, realtime::RealTimeState, save::ConfigState};
use kintaro_egui_lib::{epi::App, ScreenDescriptor};
use std::{fs::File, io::Write, thread};

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
        if let Some(ref mut controls) = self.controls {
            let mut state = controls.state.lock().unwrap();
            if state.save {
                let filename = "./save/saved.json";
                let instance_mul = state.instance_mul.to_owned();
                let camera = self.cameras.current.state();

                thread::spawn(move || {
                    let mut file = File::create(filename).unwrap();
                    let config_state = ConfigState {
                        camera,
                        instance_mul,
                    };
                    let serialized = serde_json::to_string(&config_state)
                        .unwrap_or_else(|_| panic!("unable to serialize, {}", filename));
                    file.write_all(serialized.as_bytes())
                        .expect("unable to write to file on save");
                });
                state.save = false;
                println!("Saved {}", filename);
            }
        }
    }

    pub fn render_gui(&mut self, window: &winit::window::Window, view: &wgpu::TextureView) {
        if let Some(ref mut controls) = self.controls {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("RenderPassInput Command Encoder"),
                });

            controls.platform.begin_frame();
            let previous_frame_time = self.clock.current().last_period;
            let app_output = kintaro_egui_lib::epi::backend::AppOutput::default();

            let frame =
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
                    repaint_signal: controls.repaint_signal.clone(),
                });

            controls.app.update(&controls.platform.context(), &frame);

            // End the UI frame. We could now handle the output and draw the UI with the backend.
            let (_output, paint_commands) = controls.platform.end_frame(Some(window));
            let paint_jobs = controls.platform.context().tessellate(paint_commands);

            // Upload all resources for the GPU.
            let screen_descriptor = ScreenDescriptor {
                physical_width: self.size.0,
                physical_height: self.size.1,
                scale_factor: window.scale_factor() as f32,
            };
            controls.renderpass.update_texture(
                &self.device,
                &self.queue,
                &controls.platform.context().font_image(),
            );
            controls
                .renderpass
                .update_user_textures(&self.device, &self.queue);

            controls.renderpass.update_buffers(
                &self.device,
                &self.queue,
                &paint_jobs,
                &screen_descriptor,
            );

            controls
                .renderpass
                .execute(&mut encoder, view, &paint_jobs, &screen_descriptor, None)
                .unwrap();

            self.queue.submit(Some(encoder.finish()));
        }
    }

    pub fn update_gui(&mut self, size: (u32, u32)) {
        let mut play = false;
        let mut pause = false;
        if let Some(ref controls) = self.controls {
            let s = controls.state.lock().unwrap();

            // if s.camera_index != self.cameras.index {
            // self.cameras.current = Camera::new(&self.cameras.configs[s.camera_index], size)
            // }
            if self.composition.is_some() {
                if !s.play && !self.is_paused() {
                    pause = true;
                }
                if s.play && self.is_paused() {
                    play = true;
                }

                //TODO: Send volume
                self.render_manager.lock().unwrap().update_volume(s.volume);
            }
        };
        if play {
            self.play()
        };
        if pause {
            self.pause()
        };
    }
}
