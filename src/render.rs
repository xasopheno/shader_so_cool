use wgpu::SwapChainTexture;

use crate::State;

impl State {
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let now = std::time::Instant::now();
        let dt = now - self.last_render_time;
        self.last_render_time = now;
        self.update(dt);
        let frame = self.swap_chain.get_current_frame().unwrap().output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let color_attachments = make_color_attachments(&frame, true);
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // This is what [[location(0)]] in the fragment shader targets
                color_attachments: &[color_attachments],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

fn make_color_attachments(
    frame: &SwapChainTexture,
    clear: bool,
) -> wgpu::RenderPassColorAttachment {
    if clear {
        wgpu::RenderPassColorAttachment {
            view: &frame.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.02,
                    a: 1.0,
                }),
                store: true,
            },
        }
    } else {
        wgpu::RenderPassColorAttachment {
            view: &frame.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }
    }
}
