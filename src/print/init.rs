use super::PrintState;
use crate::composition::Composition;
use crate::op_stream::renderpasses::make_renderpasses;
use crate::{
    canvas::Canvas,
    clock::{Clock, PrintClock},
    config::Config,
};

impl PrintState {
    pub async fn init(config: &mut Config) -> PrintState {
        let size = config.window_size;
        println!("{}/{}", size.0, size.1);
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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let toy = crate::toy::setup_toy(&device, size, texture_desc.format);

        let op_streams = crate::op_stream::OpStream::from_json(&config.filename);

        let renderpasses =
            make_renderpasses(&device, op_streams, &shader, config, texture_desc.format);

        PrintState {
            device,
            queue,
            size,
            clock: PrintClock::init(&config),
            count: 0,

            composition: Composition {
                config: config.clone(),
                camera: crate::camera::Camera::new(&config.cameras[0], size, &config, 0),
                renderpasses,
                toy: Some(toy),
                canvas: Canvas::init(size),
            },

            texture,
            texture_view,
            time_elapsed: std::time::Duration::from_millis(0),
        }
    }
}
