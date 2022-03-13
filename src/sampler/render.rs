use crate::frame::types::Frame;

use super::types::Sampler;

impl Sampler {
    pub fn render(
        &self,
        device: &wgpu::Device,
        input_frame: &Frame,
        output_frame: &Frame,
        // surface_texture_view: &wgpu::TextureView,
    ) {
        // let mut main_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        // label: Some("Render Encoder"),
        // });
        // {

        // let mut main_rpass = main_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        // label: Some("Render Pass"),
        // color_attachments: &[wgpu::RenderPassColorAttachment {
        // view: &surface_texture_view,
        // resolve_target: None,
        // ops: wgpu::Operations {
        // load: wgpu::LoadOp::Clear(wgpu::Color {
        // r: 0.1,
        // g: 0.04,
        // b: 0.08,
        // a: 1.0,
        // }),
        // store: true,
        // },
        // }],
        // depth_stencil_attachment: None,
        // });

        // main_rpass.set_pipeline(&frame.render_pipeline);
        // main_rpass.set_bind_group(0, &frame.texture_bind_group, &[]);
        // main_rpass.set_vertex_buffer(0, frame.vertex_buffer.slice(..));
        // // main_rpass.set_vertex_buffer(1, self.vertex_buffer.slice(..));
        // main_rpass.set_vertex_buffer(1, self.instances.slice(..));
        // main_rpass.set_index_buffer(frame.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        // main_rpass.draw_indexed(0..frame.indices.len() as u32, 0, 0..4);
        // // main_rpass.draw_indexed(0..frame.indices.len() as u32, 0, 0..1);
        // // main_rpass.draw_model(&self.model);
        // }
    }
}
