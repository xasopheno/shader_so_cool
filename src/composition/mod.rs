pub mod render;
use crate::{camera::Camera, canvas::Canvas, config::Config, shared::RenderPassInput, toy::Toy};

pub struct Composition {
    pub renderpasses: Vec<RenderPassInput>,
    pub canvas: Canvas,
    pub camera: Camera,
    pub toy: Option<Toy>,
    pub config: Config,
}
