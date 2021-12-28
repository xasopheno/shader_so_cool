use winit::window::Window;

pub struct ImageSetup {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub config: wgpu::SurfaceConfiguration,
}

pub async fn setup(window: &Window) -> ImageSetup {
    let size = window.inner_size();

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
            None, // Trace path
        )
        .await
        .unwrap();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);

    ImageSetup {
        device,
        queue,
        size,
        surface,
        config,
    }
}
