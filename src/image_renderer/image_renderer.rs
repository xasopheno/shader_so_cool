use crate::error::KintaroError;

use super::image_render::ImageRender;
use super::image_texture::ImageTexture;

pub struct ImageRenderer {
    // TODO: What else should this do?
    pub image_render: ImageRender,
}

impl ImageRenderer {
    pub async fn new<'a>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        image_path: &'a str,
    ) -> Result<Self, KintaroError> {
        let image_texture = ImageTexture::from_image(&device, &queue, image_path)?;
        let image_render = ImageRender::new(&device, format, &image_texture);

        Ok(Self { image_render })
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
    ) -> Result<(), wgpu::SurfaceError> {
        self.image_render.render_pass(device, queue, &view);

        Ok(())
    }
}
