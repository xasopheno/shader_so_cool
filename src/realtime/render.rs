use crate::{realtime::RealTimeState, shared::render_pass};

impl RealTimeState {
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.update();
        let frame = self.swap_chain.get_current_frame().unwrap().output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        render_pass(&mut encoder, &self.renderpass, &frame.view, &self.config);

        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
