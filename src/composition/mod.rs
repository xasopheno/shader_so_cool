pub mod render;
use crate::{
    camera::Camera, canvas::Canvas, config::Config, glyphy::Glyphy, image_renderer::ImageRenderer,
    shared::RenderPassInput, toy::Toy,
};

pub struct Composition {
    pub renderpasses: Vec<RenderPassInput>,
    pub canvas: Canvas,
    pub camera: Camera,
    pub toy: Option<Toy>,
    pub image_renderer: Option<ImageRenderer>,
    pub glyphy: Option<Glyphy>,
    pub config: Config,
}

impl Composition {}
