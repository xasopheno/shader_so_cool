use crate::{clock::Clock, realtime::RealTimeState};

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        self.clock.update();
        self.audio_stream_handle
            .set_volume(self.gui.state.lock().unwrap().volume);

        self.handle_save();

        let the_frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = the_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder"),
            });

        self.composition.render(
            &self.device,
            &self.queue,
            &mut encoder,
            self.size,
            &self.clock,
            self.gui.state.lock().unwrap().instance_mul,
            &view,
        );

        self.render_gui(window, &mut encoder, &view);

        // Submit the commands.
        self.queue.submit(Some(encoder.finish()));
        the_frame.present();

        self.update_gui();

        Ok(())
    }
}
