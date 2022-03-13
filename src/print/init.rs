use super::PrintState;
use crate::application::AvMap;
use crate::composition::Composition;
use crate::error::KintaroError;
use crate::frame::types::Frame;
use crate::frame::vertex::make_square_buffers; use crate::renderable::{RenderableEnum, ToRenderable};
use crate::{
    canvas::Canvas,
    clock::{Clock, PrintClock},
    config::Config,
};
use colored::*;

impl PrintState {
    pub async fn init(
        config: &mut Config<'static>,
        av_map: &AvMap,
    ) -> Result<PrintState, KintaroError> {
        todo!();
        // let size = config.window_size;
        // println!("{}", format!("Frame Size: {}/{}\n", size.0, size.1).green());
        // let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        // let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        // let adapter = instance
            // .request_adapter(&wgpu::RequestAdapterOptions {
                // power_preference: wgpu::PowerPreference::default(),
                // compatible_surface: None,
                // force_fallback_adapter: false,
            // })
            // .await
            // .unwrap();
        // let (device, queue) = adapter
            // .request_device(&Default::default(), None)
            // .await
            // .unwrap();

        // let renderable_configs = config.renderable_configs.to_owned();
        // let renderables: Vec<RenderableEnum> = renderable_configs
            // .iter()
            // .map(|renderable_config| {
                // renderable_config
                    // .to_renderable(&device, &queue, config, av_map, format)
                    // .unwrap()
            // })
            // .collect();

        // let frame = Frame::new(&device, size, format, make_square_buffers)?;

        // Ok(PrintState {
            // device,
            // queue,
            // size,
            // clock: PrintClock::init(config),
            // count: 0,

            // composition: Composition {
                // renderables,
                // config: config.clone(),
                // camera: crate::camera::Camera::new(&config.cameras[0], size, config, 0),
                // canvas: Canvas::init(size),
            // },
            // frame,
            // time_elapsed: std::time::Duration::from_millis(0),
        // })
    }
}
