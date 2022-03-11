use crate::{clock::Clock, error::KintaroError, realtime::RealTimeState};

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), KintaroError> {
        self.handle_save();
        self.clock.update();

        self.composition.render(
            &self.device,
            &self.queue,
            self.size,
            &self.clock,
            self.gui.state.lock().unwrap().instance_mul,
            &self.frame.texture.view,
        )?;

        let mut surface_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Surface Encoder"),
                });

        let surface_frame = self
            .surface
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let surface_texture_view = surface_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.surface
            .render(&mut surface_encoder, &self.frame, &surface_texture_view);

        self.queue.submit(std::iter::once(surface_encoder.finish()));
        self.render_gui(window, &surface_texture_view);

        surface_frame.present();

        self.update_gui();

        Ok(())
    }
}
