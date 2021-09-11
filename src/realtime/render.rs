use crate::{
    clock::Clock,
    realtime::RealTimeState,
    shared::{render_pass, update},
};

impl RealTimeState {
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.clock.update();
        let time = self.clock.current();
        self.camera.update(time.last_period);
        for renderpass in self.renderpasses.iter_mut() {
            renderpass.uniforms.update_view_proj(&self.camera);
        }

        let frame = self.swap_chain.get_current_frame().unwrap().output;
        // let mut encoders = vec![];
        for (n, mut renderpass) in self.renderpasses.iter_mut().enumerate() {
            update(
                time,
                &mut renderpass,
                &self.device,
                &self.queue,
                (self.size.width, self.size.height),
                &self.canvas,
            );
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            let accumulation = if n == 0 { false } else { true };

            render_pass(&mut encoder, &renderpass, &frame.view, &self.config, true);
            // if n == 1 {
            // encoders.push(encoder.finish());

            self.queue.submit(std::iter::once(encoder.finish()));
            // }
            // self.queue.submit(encoders);
        }

        Ok(())
    }
}
