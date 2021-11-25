mod input;
pub mod render;
mod resize;
pub mod setup;

use crate::canvas::Canvas;
use crate::{composition::Composition, op_stream::renderpasses::make_renderpasses};
use setup::Setup;
use weresocool::generation::parsed_to_render::AudioVisual;

use crate::{
    clock::{Clock, RenderClock},
    config::Config,
    realtime::render::ExampleRepaintSignal,
};
use futures::executor::block_on;
use winit::window::Window;

use self::setup::Gui;

pub struct RealTimeState {
    pub composition: Composition,

    pub clock: RenderClock,
    pub count: u32,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub surface: wgpu::Surface,

    pub mouse_pressed: bool,
    pub gui: Gui,
    pub repaint_signal: std::sync::Arc<ExampleRepaintSignal>,
    pub audio_stream_handle: rodio::Sink,
}

impl RealTimeState {
    pub fn init(
        window: &Window,
        config: &mut Config,
        repaint_signal: std::sync::Arc<ExampleRepaintSignal>,
        audio_stream_handle: rodio::Sink,
        av: AudioVisual,
    ) -> RealTimeState {
        let size = (config.window_size.0, config.window_size.1);
        println!("{}/{}", size.0, size.1);
        let Setup {
            device,
            surface,
            queue,
            gui,
        } = block_on(Setup::init(window, config));

        let toy = crate::toy::setup_toy(&device, size, wgpu::TextureFormat::Bgra8UnormSrgb);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let op_streams = crate::op_stream::OpStream::from_vec_op4d(av.visual, av.length);

        let renderpasses = make_renderpasses(
            &device,
            op_streams,
            &shader,
            config,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        );

        Self {
            device,
            queue,
            size,
            clock: RenderClock::init(&config),
            count: 0,
            composition: Composition {
                config: config.clone(),
                camera: crate::camera::Camera::new(&config.cameras[0], size, &config, 0),
                renderpasses,
                toy: Some(toy),
                canvas: Canvas::init((size.0, size.1)),
            },
            surface,
            gui,
            repaint_signal: repaint_signal.clone(),
            audio_stream_handle,
            mouse_pressed: false,
        }
    }

    pub fn play(&mut self) {
        self.clock.play();
        self.audio_stream_handle.play();
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.clock.pause();
        self.audio_stream_handle.pause();
    }
}
