use crate::frame::types::Frame;

use super::types::Sampler;

impl Sampler {
    pub fn render(&self, device: &wgpu::Device, frame: &Frame, output_frame: &Frame) {
        let mut main_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Sampler Encoder"),
        });
        {
            let mut rpass = main_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Sampler Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &output_frame.texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 1.00,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&frame.render_pipeline);
            rpass.set_bind_group(0, &frame.texture_bind_group, &[]);
            rpass.set_vertex_buffer(0, frame.vertex_buffer.slice(..));
            // main_rpass.set_vertex_buffer(1, self.vertex_buffer.slice(..));
            rpass.set_vertex_buffer(1, self.instances.slice(..));
            rpass.set_index_buffer(frame.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            rpass.draw_indexed(0..frame.indices.len() as u32, 0, 0..4);
            // main_rpass.draw_indexed(0..frame.indices.len() as u32, 0, 0..1);
            // main_rpass.draw_model(&self.model);
        }
    }
}
