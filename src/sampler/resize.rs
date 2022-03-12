use super::types::Sampler;

impl Sampler {
    pub fn resize(&mut self, _device: &wgpu::Device, _new_size: winit::dpi::PhysicalSize<u32>) {
        // self.texture = super::texture::Texture::new(
        // device,
        // (new_size.width, new_size.height),
        // "Surface Texture",
        // self.texture.format,
        // )
        // .unwrap();
    }
}
