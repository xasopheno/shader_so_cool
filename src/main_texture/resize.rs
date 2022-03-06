use super::types::MainTexture;

impl MainTexture {
    pub fn resize(&mut self, device: &wgpu::Device, new_size: winit::dpi::PhysicalSize<u32>) {
        self.texture = super::texture::Texture::new(
            device,
            (new_size.width, new_size.height),
            "Surface Texture",
            self.texture.format,
        )
        .unwrap();
    }
}
