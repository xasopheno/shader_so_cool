pub mod gui;
mod input;
pub mod render;
mod resize;
pub mod setup;

use std::collections::HashMap;

use crate::application::AvMap;
use crate::canvas::Canvas;
use crate::composition::Composition;
use crate::error::KintaroError;
use crate::frame::types::Frame;
use crate::frame::types::Frames;
use crate::frame::vertex::make_square_buffers;
use crate::renderable::RenderableEnum;
use crate::renderable::ToRenderable;
use crate::surface::Surface;
use setup::Setup;

use crate::{
    clock::{Clock, RenderClock},
    config::Config,
    realtime::gui::GuiRepaintSignal,
};
use futures::executor::block_on;
use winit::window::Window;

use self::setup::Gui;

pub struct RealTimeState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub surface: Surface,
    pub composition: Composition,
    pub av_map: AvMap,
    pub audio_stream_handle: Option<rodio::Sink>,

    pub clock: RenderClock,
    pub count: u32,

    pub mouse_pressed: bool,
    pub gui: Gui,
    pub repaint_signal: std::sync::Arc<GuiRepaintSignal>,
}

pub fn make_frames<'a>(
    device: &wgpu::Device,
    size: (u32, u32),
    format: wgpu::TextureFormat,
    names: Vec<&'a str>,
) -> Result<Frames, KintaroError> {
    let mut result = HashMap::new();
    names.iter().for_each(|n| {
        let frame =
            Frame::new(&device, size, format, make_square_buffers).expect("unable to make frame");
        result.insert(n.to_string(), frame);
    });

    Ok(result)
}

impl<'a> RealTimeState {
    pub fn init(
        window: &Window,
        //TODO: remove mut
        config: &mut Config<'static>,
        repaint_signal: std::sync::Arc<GuiRepaintSignal>,
        av_map: AvMap,
        audio_stream_handle: Option<rodio::Sink>,
    ) -> Result<RealTimeState, KintaroError> {
        let size = (config.window_size.0, config.window_size.1);
        println!("{}/{}", size.0, size.1);
        let Setup {
            device,
            surface,
            queue,
            gui,
            format,
        } = block_on(Setup::init(window, config))?;

        let frame_passes = config.renderable_configs.to_owned();
        let mut frame_names = vec![];
        let renderables: Vec<crate::renderable::RenderableEnum> = frame_passes
            .into_iter()
            .flat_map(|frame_pass| {
                frame_names.push(frame_pass.output_frame);
                frame_pass
                    .renderables
                    .iter()
                    .map(|renderable_config| {
                        renderable_config
                            .to_renderable(
                                &device,
                                &queue,
                                config,
                                &av_map,
                                format,
                                frame_pass.output_frame.to_string(),
                            )
                            .unwrap()
                    })
                    .collect::<Vec<RenderableEnum>>()
            })
            .collect::<Vec<RenderableEnum>>();

        let frames = make_frames(&device, size, format, frame_names)?;

        Ok(Self {
            device,
            queue,
            size,
            clock: RenderClock::init(config),
            count: 0,
            composition: Composition {
                renderables,
                config: config.clone(),
                camera: crate::camera::Camera::new(&config.cameras[0], size, config, 0),
                canvas: Canvas::init(size),
                frames,
            },
            surface,
            gui,
            repaint_signal,
            av_map,
            audio_stream_handle,
            mouse_pressed: false,
        })
    }

    pub fn play(&mut self) {
        self.clock.play();
        if let Some(a) = &self.audio_stream_handle {
            a.play()
        }
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.clock.pause();
        if let Some(a) = &self.audio_stream_handle {
            a.pause()
        }
    }
}
