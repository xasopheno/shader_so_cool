use super::image_render::ImageRender;
use super::image_texture::ImageTexture;
use super::setup::{setup, ImageSetup};
use winit::window::Window;

pub struct ImageRenderer {
    pub image_render: ImageRender,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl ImageRenderer {
    pub async fn new(window: &Window) -> Self {
        let ImageSetup {
            device,
            queue,
            size,
            surface,
            config,
            ..
        } = setup(window).await;

        let image_texture = ImageTexture::from_image(&device, &queue).unwrap();
        let image_render = ImageRender::new(&device, config.format, &image_texture);

        Self {
            image_render,
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.image_render
            .render_pass(&self.device, &self.queue, &view);

        output.present();

        Ok(())
    }
}
