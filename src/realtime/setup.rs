use crate::{
    config::Config,
    error::KintaroError,
    frame::{types::Frame, vertex::make_square_buffers},
    surface::Surface,
};
use kintaro_egui_lib::{Platform, PlatformDescriptor, RenderPass, UiState};
use std::sync::{Arc, Mutex};
use winit::window::Window;

pub struct Gui {
    pub platform: Platform,
    pub renderpass: RenderPass,
    pub app: kintaro_egui_lib::WrapApp,
    pub state: Arc<Mutex<UiState>>,
}

pub struct Setup {
    pub surface: Surface,
    pub frame: Frame,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub gui: Gui,
    pub format: wgpu::TextureFormat,
}

impl Setup {
    pub async fn init<'a>(window: &Window, config: &'a Config<'a>) -> Result<Self, KintaroError> {
        let size = config.window_size;
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::POLYGON_MODE_LINE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let format = surface.get_preferred_format(&adapter).unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let frame = Frame::new(&device, size, format, make_square_buffers)?;

        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.0,
            physical_height: size.1,
            scale_factor: window.scale_factor(),
            style: Default::default(),
            ..Default::default()
        });
        let renderpass = RenderPass::new(&device, format, 1);
        let state = Arc::new(Mutex::new(kintaro_egui_lib::UiState {
            play: true,
            save: false,
            volume: config.volume,
            camera_index: 0,
            instance_mul: config.instance_mul,
            reset: false,
        }));
        let app = kintaro_egui_lib::WrapApp::init(state.clone(), config.cameras.len());

        let surface = Surface { surface };

        Ok(Self {
            device,
            queue,
            surface,
            frame,
            format,
            gui: Gui {
                platform,
                renderpass,
                app,
                state,
            },
        })
    }
}
