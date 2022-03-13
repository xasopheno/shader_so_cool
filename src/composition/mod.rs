pub mod render;
use crate::{
    camera::Camera, canvas::Canvas, config::Config, frame::types::Frames,
    renderable::RenderableEnum,
};

pub struct Composition {
    pub renderables: Vec<RenderableEnum>,
    pub frames: Frames,
    pub canvas: Canvas,
    pub camera: Camera,
    pub config: Config<'static>,
}
