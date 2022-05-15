pub mod render;
use crate::{
    camera::Camera, camera::CameraConfig, frame::types::Frames, renderable::RenderableEnum,
};

pub struct RenderableEnums(pub Vec<RenderableEnum>);
pub struct Composition {
    pub renderables: RenderableEnums,
    pub frames: Frames,
}
