use crate::config::Config;
use crate::instance::Instance;
use crate::op_stream::OpStream;
use crate::vertex::shape::Shape;
use crate::vertex::Vertex;

use super::make_color_attachments;

pub struct RenderPassInput {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub shape: Shape,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instances: Vec<Instance>,
    pub uniforms: crate::uniforms::RealtimeUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub op_stream: OpStream,
}

impl RenderPassInput {
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
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
}
