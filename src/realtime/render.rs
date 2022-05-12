use crate::{clock::Clock, error::KintaroError, realtime::RealTimeState};

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), KintaroError> {
        // self.handle_save();
        self.clock.update();

        if let Some(controls) = &self.controls {
            todo!();
            self.composition.render(
                &self.device,
                &self.queue,
                self.size,
                &self.clock,
                controls.state.lock().unwrap().instance_mul,
            )?;
        }

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

        self.surface.render(
            &mut surface_encoder,
            &self.composition.frames.get("main").unwrap(),
            &surface_texture_view,
        );

        self.queue.submit(std::iter::once(surface_encoder.finish()));
        self.render_gui(window, &surface_texture_view);

        surface_frame.present();

        // self.update_gui(self.size);

        Ok(())
    }
}
