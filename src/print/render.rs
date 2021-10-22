use crate::toy::toy_renderpass;
use kintaro_egui_lib::InstanceMul;

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
        self.camera.update(time.last_period);

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (&self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        for renderpass in self.renderpasses.iter_mut() {
            update(
                true,
                time,
                renderpass,
                &self.device,
                &self.queue,
                (self.size.0, self.size.1),
                &self.canvas,
                self.config.instance_mul,
            );
        }

        if let Some(toy) = &mut self.toy {
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

        for (n, renderpass) in self.renderpasses.iter_mut().enumerate() {
            renderpass
                .uniforms
                .update_view_proj(view_position, view_proj);
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            let accumulation = if n == 0 { true } else { true };

            render_pass(
                &mut encoder,
                &renderpass,
                &self.texture_view,
                &self.config,
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
