use super::PrintState;
use crate::application::VisualsMap;
use crate::camera::Cameras;
use crate::composition::Composition;
use crate::error::KintaroError;
use crate::realtime::{make_frames, make_renderable_enums};
use crate::{
    canvas::Canvas,
    clock::{Clock, PrintClock},
    config::Config,
};
use colored::*;

impl PrintState {
    pub async fn init(
        config: &mut Config<'static>,
        av_map: &VisualsMap,
    ) -> Result<PrintState, KintaroError> {
        let size = config.window_size;
        println!("{}", format!("Frame Size: {}/{}\n", size.0, size.1).green());
        let format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();

        let (renderables, frame_names) =
            make_renderable_enums(&device, &queue, format, &av_map, config);

        let frames = make_frames(&device, size, format, frame_names)?;

        Ok(PrintState {
            device,
            queue,
            size,
            clock: PrintClock::init(config),
            canvas: Canvas::init(size),
            count: 0,
            cameras: Cameras {
                current: crate::camera::Camera::new(&config.cameras[0], size),
                configs: config.cameras.clone(),
                index: 0,
            },

            composition: Composition {
                frames,
                renderables,
                audio_stream_handle: None,
            },

            instance_mul: config.instance_mul,
            time_elapsed: std::time::Duration::from_millis(0),
        })
    }
}
