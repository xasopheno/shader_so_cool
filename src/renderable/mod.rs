use crate::{image_renderer::ImageRenderer, shared::RenderPassInput, toy::Toy, Config};

pub struct RenderableInput<'a> {
    pub is_playing: bool,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
    pub config: &'a Config<'a>,
    pub size: (u32, u32),
    pub total_elapsed: f32,
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
    pub clear: bool,
}
pub trait Renderable {
    fn render_pass<'a>(&mut self, input: RenderableInput<'a>) -> Result<(), wgpu::SurfaceError>;
}

impl Renderable for Toy {
    fn render_pass<'a>(&mut self, input: RenderableInput<'a>) -> Result<(), wgpu::SurfaceError> {
        self.toy_renderpass(
            input.is_playing,
            input.device,
            input.queue,
            input.view,
            input.size,
            input.total_elapsed,
            input.clear,
        )
    }
}

impl Renderable for ImageRenderer {
    fn render_pass<'a>(&mut self, input: RenderableInput<'a>) -> Result<(), wgpu::SurfaceError> {
        self.render(input.device, input.queue, input.view)
    }
}

impl<'a> Renderable for Vec<RenderPassInput> {
    fn render_pass(&mut self, input: RenderableInput) -> Result<(), wgpu::SurfaceError> {
        for renderpass in self.iter_mut() {
            renderpass
                .uniforms
                .update_view_proj(input.view_position, input.view_proj);

            renderpass.render(input.encoder, input.view, input.config, !input.clear);
        }

        Ok(())
    }
}
