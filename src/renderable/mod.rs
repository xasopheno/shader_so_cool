use crate::{image_renderer::ImageRenderer, shared::RenderPassInput, toy::Toy, Config};

pub struct RenderableInput<'a> {
    is_playing: bool,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,
    config: &'static Config<'static>,
    size: (u32, u32),
    total_elapsed: f32,
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
    clear: bool,
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

impl<'a> Renderable for Vec<RenderPassInput<'a>> {
    fn render_pass(&mut self, input: RenderableInput<'a>) -> Result<(), wgpu::SurfaceError> {
        let view_position: [f32; 4] = self.camera.position.to_homogeneous().into();
        let view_proj: [[f32; 4]; 4] =
            (self.camera.projection.calc_matrix() * self.camera.calc_matrix()).into();

        for idx in 0..self.renderpasses.len() {
            self.update(
                clock.is_playing(),
                time,
                idx,
                device,
                queue,
                size,
                instance_mul,
            );
        }

        for (n, renderpass) in self.iter_mut().enumerate() {
            renderpass
                .uniforms
                .update_view_proj(view_position, view_proj);

            // let accumulation = n > 0 || self.toy.is_some();
            renderpass.render(input.encoder, input.view, input.config, !input.clear);
        }

        Ok(())
    }
}
