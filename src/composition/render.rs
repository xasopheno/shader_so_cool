use crate::toy::toy_renderpass;
use kintaro_egui_lib::InstanceMul;
use wgpu::TextureView;

use crate::clock::Clock;
use crate::shared::update;

use super::Composition;

impl Composition {
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        clock: &impl Clock,
        instance_mul: InstanceMul,
        view: &TextureView,
    ) {
        let time = clock.current();
        self.camera.update(time.last_period);

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        for renderpass in self.renderpasses.iter_mut() {
            update(
                clock.is_playing(),
                time,
                renderpass,
                device,
                queue,
                size,
                &self.canvas,
                instance_mul,
            );
        }

        if let Some(toy) = &mut self.toy {
            toy_renderpass(true, toy, device, queue, &view, size, time.total_elapsed)
                .expect("toy error");
        }

        for (n, renderpass) in self.renderpasses.iter_mut().enumerate() {
            renderpass
                .uniforms
                .update_view_proj(view_position, view_proj);
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            let accumulation = n > 0 || self.toy.is_some();
            renderpass.render(&mut encoder, &view, &self.config, accumulation);

            queue.submit(std::iter::once(encoder.finish()));
        }
    }
}
