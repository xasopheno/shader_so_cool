mod init;
mod render;
pub mod write;
use crate::clock::PrintClock;
use crate::composition::Composition;
use crate::frame::types::Frame;
use kintaro_egui_lib::InstanceMul;

pub struct PrintState {
    pub composition: Composition,
    // pub frame: Frame,
    pub clock: PrintClock,
    pub count: u32,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub time_elapsed: std::time::Duration,

    pub instance_mul: InstanceMul,
}
