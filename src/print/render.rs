use crate::toy::toy_renderpass;

use crate::{
    clock::Clock,
    shared::{render_pass, update},
};

use super::{
    write::{copy_texture_to_buffer, write_img},
    PrintState,
};

impl PrintState {
    pub async fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.clock.update();
        let time = self.clock.current();
        self.composition.camera.update(time.last_period);

        let view_position: [f32; 4] = self.composition.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] = (&self.composition.camera.projection.calc_matrix()
            * self.composition.camera.calc_matrix())
        .into();

        for renderpass in self.composition.renderpasses.iter_mut() {
            update(
                true,
                time,
                renderpass,
                &self.device,
                &self.queue,
                (self.size.0, self.size.1),
                &self.composition.canvas,
                self.composition.config.instance_mul,
            );
        }

        if let Some(toy) = &mut self.composition.toy {
            toy_renderpass(
                self.clock.is_playing(),
                toy,
                &self.device,
                &self.queue,
                &self.texture_view,
                self.size,
                time.total_elapsed,
            )?
        }

        for (n, renderpass) in self.composition.renderpasses.iter_mut().enumerate() {
            renderpass
                .uniforms
                .update_view_proj(view_position, view_proj);
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

            let accumulation = n > 0 || self.composition.toy.is_some();

            render_pass(
                &mut encoder,
                &renderpass,
                &self.texture_view,
                &self.composition.config,
                accumulation,
            );

            self.queue.submit(std::iter::once(encoder.finish()));
        }

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
