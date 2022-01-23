pub mod render;
use crate::{camera::Camera, canvas::Canvas, config::Config, renderable::RenderableEnum};

pub struct Composition {
    pub renderables: Vec<RenderableEnum>,
    pub canvas: Canvas,
    pub camera: Camera,
    pub config: Config<'static>,
}
