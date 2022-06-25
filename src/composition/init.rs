use super::Composition;
use crate::error::KintaroError;
use crate::realtime::make_frames;
use crate::realtime::make_renderable_enums;
use crate::Config;

impl Composition {
    pub fn init_realtime(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        config: &Config<'static>,
    ) -> Result<Self, KintaroError> {
        let (renderables, frame_names) = make_renderable_enums(&device, &queue, format, config)?;

        let frames = make_frames(&device, config.window_size, format, frame_names)?;

        Ok(Composition {
            renderables,
            frames,
        })
    }
}
