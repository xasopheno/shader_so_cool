use super::PrintState;
use crate::application::VisualsMap;
use crate::composition::Composition;
use crate::error::KintaroError;
use crate::frame::types::Frame;
use crate::realtime::{make_frames, make_renderable_enums};
// use crate::frame::types::Frame;
use crate::frame::vertex::make_square_buffers;
// use crate::renderable::ToRenderable;
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

        // let frame = Frame::new(&device, size, format, make_square_buffers)?;

        let (renderables, frame_names) =
            make_renderable_enums(&device, &queue, format, &av_map, config);

        let frames = make_frames(&device, size, format, frame_names)?;

        Ok(PrintState {
            device,
            queue,
            size,
            clock: PrintClock::init(config),
            count: 0,

            composition: Composition {
                frames,
                renderables,
                camera: crate::camera::Camera::new(&config.cameras[0], size, 0),
                canvas: Canvas::init(size),
                camera_configs: config.cameras.clone(),
            },

            instance_mul: config.instance_mul,
            // frame,
            time_elapsed: std::time::Duration::from_millis(0),
        })
    }
}
