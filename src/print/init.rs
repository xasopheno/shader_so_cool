use weresocool::error::Error;
use weresocool::generation::parsed_to_render::AudioVisual;

use super::PrintState;
use crate::application::AvMap;
use crate::composition::Composition;
use crate::glyphy::Glyphy;
use crate::image_renderer::ImageRenderer;
use crate::op_stream::renderpasses::make_renderpasses;
use crate::renderable::{RenderableEnum, ToRenderable};
use crate::shader::make_shader;
use crate::{
    canvas::Canvas,
    clock::{Clock, PrintClock},
    config::Config,
};
use colored::*;

impl PrintState {
    pub async fn init(config: &mut Config<'static>, av_map: &AvMap) -> Result<PrintState, Error> {
        let size = config.window_size;
        println!("{}", format!("Frame Size: {}/{}\n", size.0, size.1).green());
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
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

        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: size.0,
                height: size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            // format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        let renderable_configs = config.renderable_configs.to_owned();
        let renderables: Vec<RenderableEnum> = renderable_configs
            .iter()
            .map(|renderable_config| {
                renderable_config
                    .to_renderable(&device, &queue, config, av_map, format)
                    .unwrap()
            })
            .collect();

        Ok(PrintState {
            device,
            queue,
            size,
            clock: PrintClock::init(&config),
            count: 0,

            composition: Composition {
                renderables,
                config: config.clone(),
                camera: crate::camera::Camera::new(&config.cameras[0], size, &config, 0),
                canvas: Canvas::init(size),
            },

            texture,
            texture_view,
            time_elapsed: std::time::Duration::from_millis(0),
        })
    }
}
