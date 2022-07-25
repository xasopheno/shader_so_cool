use kintaro_egui_lib::InstanceMul;

use crate::canvas::Canvas;
use crate::clock::ClockResult;
use crate::instance::instancer::{op4d_to_instance, prepare_op4d_to_instancer_input};
use crate::instance::{make_instance_buffer, Instance};
use crate::vertex::shape::Shape;
use crate::vertex::{make_vertex_buffer, Vertex};
use crate::Instancer;
use weresocool::generation::Op4D;

use super::make_color_attachments;

#[derive(Debug)]
pub struct RenderPassInput {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub shape: Shape,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instances: Vec<Instance>,
    pub instancer: Box<dyn Instancer>,
    pub uniforms: crate::uniforms::RealtimeUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

pub type EventStreams = std::collections::HashMap<String, RenderPassInput>;

impl RenderPassInput {
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        accumulation: bool,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            // This is what [[location(0)]] in the fragment shader targets
            color_attachments: &make_color_attachments(view, accumulation),
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(
            0..self.shape.n_indices as u32,
            0,
            0..self.instances.len() as _,
        );
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        clock_result: ClockResult,
        canvas: &Canvas,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        size: (u32, u32),
        instance_mul: InstanceMul,
        ops: &Vec<Op4D>,
    ) {
        self.vertex_buffer = make_vertex_buffer(device, self.vertices.as_slice());

        if clock_result.is_playing {
            add_new_instances_to_render_pass(self, &canvas, instance_mul, ops);
            update_instances(&clock_result, self, device, size);
        }
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}

fn add_new_instances_to_render_pass(
    renderpass: &mut RenderPassInput,
    canvas: &Canvas,
    mul: InstanceMul,
    ops: &Vec<Op4D>,
) {
    let mut new_instances = ops
        .into_iter()
        .map(|op| {
            let input = prepare_op4d_to_instancer_input(&mul, &op);
            let transformation = renderpass.instancer.op4d_to_instance_transformation(input);
            op4d_to_instance(transformation, &op, canvas)
        })
        .collect();

    renderpass.instances.append(&mut new_instances);
}

fn update_instances(
    clock_result: &ClockResult,
    renderpass: &mut RenderPassInput,
    device: &wgpu::Device,
    size: (u32, u32),
) {
    renderpass.instances.iter_mut().for_each(|i| {
        renderpass
            .instancer
            .update_instance(i, clock_result.last_period)
    });

    renderpass.instances.retain(|i| i.life > 0.0);
    renderpass.instance_buffer =
        make_instance_buffer(&renderpass.instances, (size.0, size.1), device);
}
