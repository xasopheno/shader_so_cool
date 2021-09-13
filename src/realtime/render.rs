use crate::{
    clock::Clock,
    realtime::RealTimeState,
    shared::{render_pass, update},
};

impl RealTimeState {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
        // let mut encoders = vec![];
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

        Ok(())
    }
}
