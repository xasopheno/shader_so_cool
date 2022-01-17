pub mod render;
use crate::{
    camera::Camera, canvas::Canvas, config::Config, glyphy::Glyphy, image_renderer::ImageRenderer,
    shared::RenderPassInput, toy::Toy,
};

pub struct Composition<'a> {
    pub renderpasses: Vec<RenderPassInput>,
    pub canvas: Canvas,
    pub camera: Camera,
    pub toy: Option<Toy>,
    pub image_renderer: Option<ImageRenderer>,
    pub glyphy: Option<Glyphy>,
    pub config: Config<'a>,
}

pub trait Renderable {
    fn render_pass(
        _is_playing: bool,
        toy: &mut Toy,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view: &wgpu::TextureView,
        size: (u32, u32),
        total_elapsed: f32,
        clear: bool,
    ) {
    }
}

// impl Composition {}
