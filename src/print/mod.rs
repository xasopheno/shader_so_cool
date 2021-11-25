mod init;
mod render;
mod update;
mod write;
use crate::clock::PrintClock;
use crate::composition::Composition;

pub struct PrintState {
    pub composition: Composition,
    pub clock: PrintClock,
    pub size: (u32, u32),
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub count: u32,
    pub time_elapsed: std::time::Duration,
}
