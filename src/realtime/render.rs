use crate::{
    clock::Clock, error::KintaroError, frame::instance::make_instances, realtime::RealTimeState,
};

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

        // self.frame.instances = make_instances(&self.device);

        let finish = self.surface.render(&mut surface_encoder, &self.frame);

        // self.render_gui(window);

        self.queue.submit(std::iter::once(surface_encoder.finish()));

        finish();

        self.update_gui();

        Ok(())
    }
}
