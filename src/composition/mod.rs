pub mod render;
use crate::{
    camera::Camera, canvas::Canvas, config::Config, glyphy::Glyphy, image_renderer::ImageRenderer,
    renderable::RenderableEnum, shared::RenderPassInput, toy::Toy,
};

pub struct Composition {
    pub renderables: Vec<RenderableEnum>,
    pub canvas: Canvas,
    pub camera: Camera,
    pub config: Config<'static>,
}
