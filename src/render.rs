use crate::config::Config;
use crate::helpers::make_color_attachments;
use crate::state::RenderPassInput;
use crate::State;

impl State {
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        self.update();

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        render_pass(
            &mut encoder,
            &self.renderpass,
            &self.swap_chain.get_current_frame().unwrap().output.view,
            &self.config,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

pub fn render_pass<'a>(
    encoder: &mut wgpu::CommandEncoder,
    input: &'a RenderPassInput,
    view: &wgpu::TextureView,
    config: &Config,
) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        // This is what [[location(0)]] in the fragment shader targets
        color_attachments: &make_color_attachments(view, config.accumulation),
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&input.render_pipeline);
    render_pass.set_bind_group(0, &input.uniform_bind_group, &[]);
    render_pass.set_vertex_buffer(0, input.vertex_buffer.slice(..));
    render_pass.set_vertex_buffer(1, input.instance_buffer.slice(..));
    render_pass.set_index_buffer(input.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..input.num_indices, 0, 0..input.instances.len() as _);
}
