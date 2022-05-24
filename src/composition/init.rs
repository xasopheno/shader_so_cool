use super::Composition;
use crate::application::utils::audios_and_visuals_from_frame_passes;
use crate::application::utils::sum_all_waveforms;
use crate::error::KintaroError;
use crate::realtime::make_frames;
use crate::realtime::make_renderable_enums;
use crate::realtime::Watchers;
use crate::renderable::ToRenderable;
use crate::Config;
use rodio::OutputStream;

impl Composition {
    pub fn init_realtime(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        config: &Config<'static>,
    ) -> Result<(Self, Watchers), KintaroError> {
        let (audios, visuals_map) = audios_and_visuals_from_frame_passes(&config.frame_passes)?;
        let mut audio_stream: Option<OutputStream> = None;
        let mut audio_stream_handle: Option<rodio::Sink> = None;
        if audios.len() > 0 {
            let a = sum_all_waveforms(audios);
            let (s, s_h) = crate::audio::setup_audio(&config, &a);
            audio_stream = Some(s);
            audio_stream_handle = Some(s_h);
        }

        let (renderables, frame_names) =
            make_renderable_enums(&device, &queue, format, &visuals_map, config);

        let frames = make_frames(&device, config.window_size, format, frame_names)?;

        let watchable_paths: Vec<String> = config
            .frame_passes
            .iter()
            .map(|frame_pass| {
                frame_pass
                    .renderables
                    .iter()
                    .map(|r| r.watchable_paths())
                    .flatten()
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect();

        let watchers = Watchers::init(watchable_paths)?;

        Ok((
            Composition {
                renderables,
                frames,
                audio_stream_handle,
                audio_stream,
            },
            watchers,
        ))
    }
}
