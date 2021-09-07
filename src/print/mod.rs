mod init;
mod render;
mod update;
mod write;
use crate::{
    camera::{Camera, CameraController, Projection},
    clock::PrintClock,
    config::Config,
    realtime::{Canvas, RenderPassInput},
    render_op::OpStream,
};

pub struct PrintState {
    pub clock: PrintClock,
    pub config: Config,
    pub renderpass: RenderPassInput,
    pub size: (u32, u32),
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub count: u32,
    pub op_stream: OpStream,
    pub time_elapsed: std::time::Duration,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub projection: Projection,
    pub canvas: Canvas,
    pub clear_color: (f64, f64, f64),
}
