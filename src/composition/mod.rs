pub mod render;
use crate::{
    camera::Camera, canvas::Canvas, config::Config, glyphy::Glyphy, image_renderer::ImageRenderer,
    shared::RenderPassInput, toy::Toy,
};

pub struct Composition {
    pub renderpasses: Vec<RenderPassInput>,
    pub canvas: Canvas,
    pub camera: Camera,
    pub toy: Toy,
    pub image_renderer: ImageRenderer,
    pub glyphy: Glyphy,
    pub config: Config<'static>,
}

// impl Composition {}
