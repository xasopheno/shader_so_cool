use std::sync::{Arc, Mutex};

use crate::config::Config;
use egui::FontDefinitions;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use kintaro_egui_lib::{InstanceMul, UiState};
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
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
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
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });
        let renderpass = RenderPass::new(&device, surface_format, 1);
        let state = Arc::new(Mutex::new(kintaro_egui_lib::UiState {
            play: true,
            volume: config.volume,
            camera_index: 4,
            instance_mul: InstanceMul::default(),
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
