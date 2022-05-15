pub mod render;
use crate::{
    camera::Camera, canvas::Canvas, config::CameraConfig, frame::types::Frames,
    renderable::RenderableEnum,
};
use kintaro_egui_lib::InstanceMul;

pub struct RenderableEnums(pub Vec<RenderableEnum>);
pub struct Composition {
    pub renderables: RenderableEnums,
    pub frames: Frames,

    // should live with a given event_stream
    // also instance_mul
    pub camera_configs: Vec<CameraConfig>,
    pub camera: Camera,
    pub canvas: Canvas,
}
