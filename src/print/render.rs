use crate::{clock::Clock, error::KintaroError};

use super::{
    write::{copy_texture_to_buffer, write_img},
    PrintState,
};

impl PrintState {
    pub async fn render(&mut self) -> Result<(), KintaroError> {
        self.clock.update();

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.composition.render(
            &self.device,
            &self.queue,
            self.size,
            &self.clock,
            self.instance_mul,
            &self.canvas,
            &mut self.cameras,
        )?;

        let output_buffer = copy_texture_to_buffer(
            &mut encoder,
            self.size,
            &self.device,
            &self.composition.frames.get("main").unwrap().texture.texture,
        );

        self.queue.submit(Some(encoder.finish()));

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
