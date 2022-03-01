use crate::{
    clock::Clock,
    error::KintaroError,
    print::write::{copy_image_copy_buffer_to_buffer, copy_texture_to_buffer},
    realtime::RealTimeState,
};

impl RealTimeState {
    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), KintaroError> {
        self.clock.update();

        if let Some(a) = &self.audio_stream_handle {
            a.set_volume(self.gui.state.lock().unwrap().volume);
        };

        self.handle_save();

        let the_frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = the_frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.composition.render(
            &self.device,
            &self.queue,
            self.size,
            &self.clock,
            self.gui.state.lock().unwrap().instance_mul,
            &view,
        )?;

        self.render_gui(window, &view);

        the_frame.present();

        self.update_gui();

        Ok(())
    }
}
