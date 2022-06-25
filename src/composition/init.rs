use super::Composition;
use crate::application::utils::audios_and_visuals_from_frame_passes;
use crate::application::utils::sum_all_waveforms;
use crate::error::KintaroError;
use crate::realtime::make_frames;
use crate::realtime::make_renderable_enums;
use crate::realtime::Watcher;
use crate::renderable::ToRenderable;
use crate::Config;
use rodio::OutputStream;

impl Composition {
    pub fn init_realtime(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        config: &Config<'static>,
    ) -> Result<Self, KintaroError> {
        let (renderables, frame_names) = make_renderable_enums(&device, &queue, format, config)?;

        let frames = make_frames(&device, config.window_size, format, frame_names)?;

        // let watchable_paths: Vec<String> = config
        // .frame_passes
        // .iter()
        // .map(|frame_pass| {
        // frame_pass
        // .renderables
        // .iter()
        // .map(|r| r.watchable_paths())
        // .flatten()
        // .collect::<Vec<String>>()
        // })
        // .flatten()
        // .collect();

        // let watchers = Watcher::init(watchable_paths.clone())?;
        // println!("Created {} watchers", watchable_paths.len());

        Ok(
            Composition {
                renderables,
                frames,
            }, // watchers,
        )
    }
}
