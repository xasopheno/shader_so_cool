use crate::frame::{instance::make_instances, types::Frame};

pub struct Surface {
    pub surface: wgpu::Surface,
}

impl Surface {
    pub fn render(
        &self,
        main_encoder: &mut wgpu::CommandEncoder,
        frame: &Frame,
        surface_texture_view: &wgpu::TextureView,
    ) {
        {
            let mut main_rpass = main_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.04,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            main_rpass.set_pipeline(&frame.render_pipeline);
            main_rpass.set_bind_group(0, &frame.texture_bind_group, &[]);
            main_rpass.set_vertex_buffer(0, frame.vertex_buffer.slice(..));
            // main_rpass.set_vertex_buffer(1, self.surface_vertex_buffer.slice(..));
            main_rpass.set_vertex_buffer(1, frame.instances.slice(..));
            main_rpass.set_index_buffer(frame.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            main_rpass.draw_indexed(0..frame.indices.len() as u32, 0, 0..4);
            // main_rpass.draw_model(&self.model);
        }
    }
}
