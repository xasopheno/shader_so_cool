use std::sync::{Arc, Mutex};

use crate::config::Config;
use kintaro_egui_lib::{Platform, PlatformDescriptor, RenderPass, UiState};
use winit::window::Window;

pub struct Gui {
    pub platform: Platform,
    pub renderpass: RenderPass,
    pub app: kintaro_egui_lib::WrapApp,
    pub state: Arc<Mutex<UiState>>,
}

pub struct Setup {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub gui: Gui,
}

impl Setup {
    pub async fn init(window: &Window, config: &Config) -> Self {
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
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let surface_format = surface.get_preferred_format(&adapter).unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.0,
            physical_height: size.1,
            scale_factor: window.scale_factor(),
            style: Default::default(),
            ..Default::default()
        });
        let renderpass = RenderPass::new(&device, surface_format, 1);
        let state = Arc::new(Mutex::new(kintaro_egui_lib::UiState {
            play: true,
            save: false,
            volume: config.volume,
            camera_index: 0,
            instance_mul: config.instance_mul,
            reset: false,
        }));
        let app = kintaro_egui_lib::WrapApp::init(state.clone(), config.cameras.len());

        Self {
            surface,
            device,
            queue,
            gui: Gui {
                platform,
                renderpass,
                app,
                state,
            },
        }
    }
}
