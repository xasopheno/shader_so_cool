use crate::clock::Clock;

use super::{
    write::{copy_texture_to_buffer, write_img},
    PrintState,
};

impl PrintState {
    pub async fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.clock.update();

        self.composition.render(
            &self.device,
            &self.queue,
            self.size,
            &self.clock,
            self.composition.config.instance_mul,
            &self.texture_view,
        );

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

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
        Ok(())
    }
}
