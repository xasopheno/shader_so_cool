mod init;
mod render;
pub mod write;
use crate::camera::Cameras;
use crate::canvas::Canvas;
use crate::clock::PrintClock;
use crate::composition::Composition;
use kintaro_egui_lib::InstanceMul;

pub struct PrintState {
    pub composition: Composition,
    pub clock: PrintClock,
    pub canvas: Canvas,
    pub count: u32,

    pub cameras: Cameras,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub time_elapsed: std::time::Duration,

    pub instance_mul: InstanceMul,
}
