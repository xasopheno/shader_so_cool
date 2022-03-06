mod init;
mod render;
pub mod write;
use crate::clock::PrintClock;
use crate::composition::Composition;
use crate::main_texture::types::MainTexture;

pub struct PrintState {
    pub composition: Composition,
    pub main_texture: MainTexture,

    pub clock: PrintClock,
    pub count: u32,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    // pub texture: wgpu::Texture,
    // pub texture_view: wgpu::TextureView,
    pub time_elapsed: std::time::Duration,
}
