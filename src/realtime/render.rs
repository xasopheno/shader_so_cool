use crate::{
    clock::Clock, error::KintaroError, main_texture::vertex::MAIN_TEXTURE_INDICES,
    realtime::RealTimeState,
};

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), KintaroError> {
        self.clock.update();

        if let Some(a) = &self.audio_stream_handle {
            a.set_volume(self.gui.state.lock().unwrap().volume);
        };

        self.handle_save();

        let surface_frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let surface_texture_view = surface_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.composition.render(
            &self.device,
            &self.queue,
            self.size,
            &self.clock,
            self.gui.state.lock().unwrap().instance_mul,
            &self.main_texture.texture.view,
        )?;

        let mut main_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        let main_texture_bind_group = crate::main_texture::setup::make_main_texture_bind_group(
            &self.device,
            &self.main_texture.texture_bind_group_layout,
            &self.main_texture.texture,
        );

        {
            let mut main_rpass = main_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            main_rpass.set_pipeline(&self.main_texture.render_pipeline);
            main_rpass.set_bind_group(0, &main_texture_bind_group, &[]);
            main_rpass.set_vertex_buffer(0, self.main_texture.vertex_buffer.slice(..));
            // main_rpass.set_vertex_buffer(1, self.surface_vertex_buffer.slice(..));
            main_rpass.set_index_buffer(
                self.main_texture.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            main_rpass.draw_indexed(0..MAIN_TEXTURE_INDICES.len() as u32, 0, 0..1);
            // main_rpass.draw_model(&self.model);
        }

        self.queue.submit(std::iter::once(main_encoder.finish()));

        self.render_gui(window, &surface_texture_view);

        surface_frame.present();

        self.update_gui();

        Ok(())
    }
}
