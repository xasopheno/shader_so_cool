use crate::config::Config;
use egui::FontDefinitions;
use egui_wgpu_backend::RenderPass;
use egui_winit_platform::{Platform, PlatformDescriptor};
use epi::*;
use winit::window::Window;

pub struct Gui {
    pub platform: Platform,
    pub renderpass: RenderPass,
    pub app: egui_demo_lib::WrapApp,
}

pub struct Setup {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub gui: Gui,
}

impl Setup {
    pub async fn init(window: &Window, _config: &Config) -> Self {
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // We use the egui_winit_platform crate as the platform.
        let platform = Platform::new(PlatformDescriptor {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: window.scale_factor(),
            font_definitions: FontDefinitions::default(),
            style: Default::default(),
        });

        // We use the egui_wgpu_backend crate as the render backend.
        let renderpass = RenderPass::new(&device, surface_format, 1);

        // Display the demo application that ships with egui.
        let app = egui_demo_lib::WrapApp::default();

        Self {
            surface,
            device,
            queue,
            gui: Gui {
                platform,
                renderpass,
                app,
            },
        }
    }
}
