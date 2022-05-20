use super::Composition;
use crate::application::utils::audios_and_visuals_from_frame_passes;
use crate::application::utils::sum_all_waveforms;
use crate::error::KintaroError;
use crate::realtime::make_frames;
use crate::realtime::make_renderable_enums;
use crate::Config;
use rodio::OutputStream;

impl Composition {
    pub fn init_realtime(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        config: &Config<'static>,
    ) -> Result<Self, KintaroError> {
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

        Ok(Composition {
            renderables,
            frames,
            audio_stream_handle,
            audio_stream,
        })
    }
}
