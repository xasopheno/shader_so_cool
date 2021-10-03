use crate::config::Config;
use crate::instance::Instance;
use crate::render_op::OpStream;
use crate::vertex::Vertex;

use super::make_color_attachments;

pub struct RenderPassInput {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub vertices_fn: fn() -> Vec<Vertex>,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instances: Vec<Instance>,
    pub uniforms: crate::uniforms::RealtimeUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub indices_fn: fn(u16) -> Vec<u16>,
    pub num_indices: u32,
    pub op_stream: OpStream,
}

pub fn render_pass<'a>(
    encoder: &mut wgpu::CommandEncoder,
    input: &'a RenderPassInput,
    view: &wgpu::TextureView,
    config: &Config,
    accumulation: bool,
) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        // This is what [[location(0)]] in the fragment shader targets
        color_attachments: &make_color_attachments(view, accumulation || config.accumulation),
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&input.render_pipeline);
    render_pass.set_bind_group(0, &input.uniform_bind_group, &[]);
    render_pass.set_vertex_buffer(0, input.vertex_buffer.slice(..));
    render_pass.set_vertex_buffer(1, input.instance_buffer.slice(..));
    render_pass.set_index_buffer(input.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..input.num_indices, 0, 0..input.instances.len() as _);
}
