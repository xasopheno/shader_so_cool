use std::{fs::File, io::Write, thread};

use crate::{
    camera::Camera,
    clock::Clock,
    realtime::RealTimeState,
    save::ConfigState,
    shared::{render_pass, update},
    toy::toy_renderpass,
};
use egui_wgpu_backend::ScreenDescriptor;
use epi::*;

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
        self.audio_stream_handle
            .set_volume(self.gui.state.lock().unwrap().volume);

        {
            let mut state = self.gui.state.lock().unwrap();
            if state.save {
                let filename = "../kintaro/saved.json";
                let instance_mul = state.instance_mul.to_owned();
                let camera = self.camera.current_state().clone();
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

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (&self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        for renderpass in self.renderpasses.iter_mut() {
            update(
                self.clock.is_playing(),
                time,
                renderpass,
                &self.device,
                &self.queue,
                self.size.into(),
                &self.canvas,
                self.gui.state.lock().unwrap().instance_mul,
            );
        }

        let output = self.surface.get_current_frame()?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        if let Some(toy) = &mut self.toy {
            toy_renderpass(
                true,
                toy,
                &self.device,
                &self.queue,
                &view,
                self.size.into(),
                time.total_elapsed,
            )
            .expect("toy error");
        }

        for (n, renderpass) in self.renderpasses.iter_mut().enumerate() {
            renderpass
                .uniforms
                .update_view_proj(view_position, view_proj);
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let accumulation = n > 0 || self.toy.is_some();
            render_pass(&mut encoder, &renderpass, &view, &self.config, accumulation);

            self.queue.submit(std::iter::once(encoder.finish()));
        }

        //TODO: Move to another file
        self.gui.platform.begin_frame();
        let previous_frame_time = time.last_period;
        let mut app_output = epi::backend::AppOutput::default();

        let mut frame = epi::backend::FrameBuilder {
            info: epi::IntegrationInfo {
                web_info: None,
                seconds_since_midnight: None,
                cpu_usage: Some(previous_frame_time),
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
            .execute(&mut encoder, &view, &paint_jobs, &screen_descriptor, None)
            .unwrap();

        // // Submit the commands.
        self.queue.submit(std::iter::once(encoder.finish()));

        {
            let s = self.gui.state.lock().unwrap();
            self.audio_stream_handle.set_volume(s.volume);
            if s.camera_index != self.camera.index {
                self.camera = Camera::new(
                    &self.config.cameras[s.camera_index],
                    self.config.window_size,
                    &self.config,
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

        Ok(())
    }
}
