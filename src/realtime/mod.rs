pub mod gui;
mod input;
pub mod render;
mod resize;
pub mod setup;

use crate::application::AudioStreamHandles;
use crate::application::AvMap;
use crate::canvas::Canvas;
use crate::glyphy::Glyphy;
use crate::image_renderer::ImageRenderer;
use crate::renderable::RenderableEnum;
use crate::renderable::ToRenderable;
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
    pub av_map: AvMap,
    pub audio_stream_handles: AudioStreamHandles,

    pub clock: RenderClock,
    pub count: u32,

    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub surface: wgpu::Surface,

    pub mouse_pressed: bool,
    pub gui: Gui,
    pub repaint_signal: std::sync::Arc<GuiRepaintSignal>,
}

impl<'a> RealTimeState {
    pub fn init(
        window: &Window,
        //TODO: remove mut
        config: &mut Config<'static>,
        repaint_signal: std::sync::Arc<GuiRepaintSignal>,
        av_map: AvMap,
        audio_stream_handles: AudioStreamHandles,
    ) -> Result<RealTimeState, Error> {
        let size = (config.window_size.0, config.window_size.1);
        println!("{}/{}", size.0, size.1);
        let Setup {
            device,
            surface,
            queue,
            gui,
        } = block_on(Setup::init(window, config));

        let renderable_configs = config.renderable_configs.to_owned();
        let renderables: Vec<RenderableEnum> = renderable_configs
            .iter()
            .map(|renderable_config| {
                renderable_config
                    .to_renderable(&device, &queue, config, &av_map)
                    .unwrap()
            })
            .collect();

        Ok(Self {
            device,
            queue,
            size,
            clock: RenderClock::init(&config),
            count: 0,
            composition: Composition {
                renderables,
                config: config.clone(),
                camera: crate::camera::Camera::new(&config.cameras[0], size, &config, 0),
                canvas: Canvas::init(size),
            },
            surface,
            gui,
            repaint_signal: repaint_signal.clone(),
            av_map,
            audio_stream_handles,
            mouse_pressed: false,
        })
    }

    pub fn play(&mut self) {
        self.clock.play();
        self.audio_stream_handles
            .iter_mut()
            .for_each(|stream_handle| stream_handle.play())
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.clock.pause();
        self.audio_stream_handles
            .iter_mut()
            .for_each(|stream_handle| stream_handle.pause())
    }
}
