use crate::clock::ClockResult;
use crate::composition::Canvas;
use crate::instance::instancer::{op4d_to_instance, prepare_op4d_to_instancer_input, Instancer};
use crate::instance::{make_instance_buffer, Instance};
use crate::renderable::{Renderable, RenderableInput};
use crate::shared::RenderPassInput;
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
        let clock_result = clock.current();
        self.camera.update(clock_result.last_period);

        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        // for idx in 0..self.renderpasses.len() {
        // self.update(idx, clock_result, device, queue, size, instance_mul);
        // }

        let render_input = RenderableInput {
            device,
            queue,
            view,
            clock_result,
            canvas: &self.canvas,
            config: &self.config,
            size,
            view_position,
            view_proj,
            instance_mul,
            clear: false,
        };

        let mut renderables: Vec<Box<&mut dyn Renderable>> = vec![
            Box::new(&mut self.image_renderer),
            Box::new(&mut self.toy),
            Box::new(&mut self.renderpasses),
        ];

        for renderable in renderables.iter_mut() {
            renderable.update(&render_input).unwrap();
            renderable.render_pass(&render_input).unwrap();
        }

        self.glyphy.render(device, queue, size, view, false)
    }

    // pub fn update(
    // &mut self,
    // idx: usize,
    // clock_result: ClockResult,
    // device: &wgpu::Device,
    // queue: &wgpu::Queue,
    // size: (u32, u32),
    // instance_mul: InstanceMul,
    // ) {
    // if clock_result.frame_count % 1000 == 0 {
    // // renderpass.vertices = renderpass.shape.gen().vertices;
    // // self.clear_color = crate::helpers::new_random_clear_color();
    // }
    // self.renderpasses[idx].vertex_buffer =
    // make_vertex_buffer(device, self.renderpasses[idx].vertices.as_slice());

    // if clock_result.is_playing {
    // update_instances(
    // &clock_result,
    // &mut self.renderpasses[idx],
    // &self.canvas,
    // device,
    // &*self.config.instancer,
    // size,
    // instance_mul,
    // );
    // }
    // // renderpass.vertices.iter_mut().for_each(|v| v.update());
    // queue.write_buffer(
    // &self.renderpasses[idx].uniform_buffer,
    // 0,
    // bytemuck::cast_slice(&[self.renderpasses[idx].uniforms]),
    // );
    // }
}
