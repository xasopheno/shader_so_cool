mod init;
mod render;
mod update;
mod write;
use crate::{
    camera::Camera, canvas::Canvas, clock::PrintClock, config::Config, render_op::OpStream,
    shared::RenderPassInput,
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
    pub canvas: Canvas,
    pub clear_color: (f64, f64, f64),
}
