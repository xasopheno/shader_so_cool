use crate::clock::ClockResult;
use crate::composition::Canvas;
use crate::instance::instancer::{prepare_op4d_to_instancer_input, Instancer};
use crate::instance::{make_instance_buffer, Instance};
use crate::shared::RenderPassInput;
use crate::toy::toy_renderpass;
use crate::vertex::make_vertex_buffer;
use kintaro_egui_lib::InstanceMul;
use wgpu::TextureView;

use crate::clock::Clock;

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

        for idx in 0..self.renderpasses.len() {
            self.update(
                clock.is_playing(),
                time,
                idx,
                device,
                queue,
                size,
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

    pub fn update(
        &mut self,
        is_playing: bool,
        time: ClockResult,
        idx: usize,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        instance_mul: InstanceMul,
    ) {
        if time.frame_count % 1000 == 0 {
            // renderpass.vertices = renderpass.shape.gen().vertices;
            // self.clear_color = crate::helpers::new_random_clear_color();
        }
        self.renderpasses[idx].vertex_buffer =
            make_vertex_buffer(device, self.renderpasses[idx].vertices.as_slice());

        if is_playing {
            update_instances(
                &time,
                &mut self.renderpasses[idx],
                &self.canvas,
                device,
                &*self.config.instancer,
                size,
                instance_mul,
            );
        }
        // renderpass.vertices.iter_mut().for_each(|v| v.update());
        queue.write_buffer(
            &self.renderpasses[idx].uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.renderpasses[idx].uniforms]),
        );
    }
}

fn update_instances(
    time: &ClockResult,
    renderpass: &mut RenderPassInput,
    canvas: &Canvas,
    device: &wgpu::Device,
    instancer: &(impl Instancer + ?Sized),
    size: (u32, u32),
    mul: InstanceMul,
) {
    let mut new_instances: Vec<Instance> = renderpass
        .op_stream
        .get_batch(time.total_elapsed)
        .into_iter()
        .map(|op| {
            let input = prepare_op4d_to_instancer_input(&mul, &op, canvas.n_row, canvas.n_column);
            instancer.op4d_to_instance(input, &op, canvas.instance_displacement)
        })
        .collect();

    renderpass.instances.append(&mut new_instances);
    renderpass.instances.iter_mut().for_each(|i| {
        // i.
        instancer.update_instance(i, time.last_period)
    });

    renderpass.instances.retain(|i| i.life > 0.0);
    renderpass.instance_buffer =
        make_instance_buffer(&renderpass.instances, (size.0, size.1), &device);
}
