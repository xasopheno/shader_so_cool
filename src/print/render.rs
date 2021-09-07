use crate::shared::render_pass;

use super::{
    write::{copy_texture_to_buffer, write_img},
    PrintState,
};

impl PrintState {
    pub async fn render(&mut self) {
        self.update();

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        render_pass(
            &mut encoder,
            &self.renderpass,
            &self.texture_view,
            &self.config,
        );

        let output_buffer =
            copy_texture_to_buffer(&mut encoder, self.size, &self.device, &self.texture);

        self.queue.submit(std::iter::once(encoder.finish()));

        write_img(
            output_buffer,
            self.clock.frame_count,
            self.size,
            &self.device,
        )
        .await;
    }
}
