pub mod render;
use crate::{frame::types::Frames, renderable::RenderableEnum};

pub struct RenderableEnums(pub Vec<RenderableEnum>);
pub struct Composition {
    pub renderables: RenderableEnums,
    pub frames: Frames,
    pub audio_stream_handle: Option<rodio::Sink>,
}
