use crate::helpers::make_color_attachments;
use crate::renderable::Renderable;
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

        render_pass(&mut encoder, &self, &frame.view);
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

pub fn render_pass<'a>(
    encoder: &mut wgpu::CommandEncoder,
    state: &'a State,
    view: &wgpu::TextureView,
) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        // This is what [[location(0)]] in the fragment shader targets
        color_attachments: &[make_color_attachments(view, state.config.accumulation)],
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&state.render_pipeline);
    render_pass.set_bind_group(0, &state.uniform_bind_group, &[]);
    render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
    render_pass.set_vertex_buffer(1, state.instance_buffer.slice(..));
    render_pass.set_index_buffer(state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..state.num_indices, 0, 0..state.instances.len() as _);
}
