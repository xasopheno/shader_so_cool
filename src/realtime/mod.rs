pub mod gui;
mod input;
pub mod render;
mod resize;
pub mod setup;

use crate::canvas::Canvas;
use crate::glyphy::Glyphy;
use crate::image_renderer::ImageRenderer;
use crate::shader::make_shader;
use crate::{composition::Composition, op_stream::renderpasses::make_renderpasses};
use setup::Setup;
use weresocool::error::Error;
use weresocool::generation::parsed_to_render::AudioVisual;

use crate::{
    clock::{Clock, RenderClock},
    config::Config,
    realtime::gui::GuiRepaintSignal,
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
    pub repaint_signal: std::sync::Arc<GuiRepaintSignal>,
    pub audio_stream_handle: rodio::Sink,
}

impl<'a> RealTimeState {
    pub fn init(
        window: &Window,
        config: &mut Config<'static>,
        repaint_signal: std::sync::Arc<GuiRepaintSignal>,
        audio_stream_handle: rodio::Sink,
        av: &AudioVisual,
    ) -> Result<RealTimeState, Error> {
        let size = (config.window_size.0, config.window_size.1);
        println!("{}/{}", size.0, size.1);
        let Setup {
            device,
            surface,
            queue,
            gui,
        } = block_on(Setup::init(window, config));

        let instance_shader = make_shader(&device, &config.instance_shader)?;
        let toy_shader = make_shader(&device, &config.toy_shader)?;

        let toy = crate::toy::setup_toy(
            &device,
            toy_shader,
            size,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        );

        let op_streams = crate::op_stream::OpStream::from_vec_op4d(av);

        let renderpasses = make_renderpasses(
            &device,
            op_streams,
            &instance_shader,
            config,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        );

        let image_renderer = pollster::block_on(ImageRenderer::new(
            &device,
            &queue,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        ));

        let glyphy = Glyphy::init(
            &device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            config.text.as_ref().unwrap().to_vec(),
        )
        .expect("Unable to setup Glyphy");

        Ok(Self {
            device,
            queue,
            size,
            clock: RenderClock::init(&config),
            count: 0,
            composition: Composition {
                glyphy,
                config: config.clone(),
                camera: crate::camera::Camera::new(&config.cameras[0], size, &config, 0),
                renderpasses,
                toy,
                canvas: Canvas::init(size),
                image_renderer,
            },
            surface,
            gui,
            repaint_signal: repaint_signal.clone(),
            audio_stream_handle,
            mouse_pressed: false,
        })
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
