use crate::op_stream::GetOps;
use crate::{clock::Clock, error::KintaroError, realtime::RealTimeState};
use weresocool::generation::json::Op4D;

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), KintaroError> {
        self.handle_save();
        self.clock.update();

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

        if let Some(ref mut composition) = self.composition {
            let instance_mul = if let Some(ref controls) = self.controls {
                controls.state.lock().unwrap().instance_mul
            } else {
                self.base_instance_mul
            };

            self.receiver.receive();

            composition.render(
                &self.device,
                &self.queue,
                self.size,
                &self.clock,
                instance_mul,
                &self.canvas,
                &mut self.cameras,
                &mut self.receiver,
            )?;

            self.surface.render(
                &mut surface_encoder,
                &composition.frames.get("main").unwrap(),
                &surface_texture_view,
            );
        }

        self.queue.submit(std::iter::once(surface_encoder.finish()));
        self.render_gui(window, &surface_texture_view);

        surface_frame.present();

        self.update_gui(self.size);

        Ok(())
    }
}
