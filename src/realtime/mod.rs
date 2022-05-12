pub mod gui;
mod input;
pub mod render;
mod resize;
pub mod setup;

use std::collections::HashMap;

use crate::application::VisualsMap;
use crate::canvas::Canvas;
use crate::composition::Composition;
use crate::composition::RenderableEnums;
use crate::error::KintaroError;
use crate::frame::types::Frame;
use crate::frame::types::Frames;
use crate::frame::vertex::make_square_buffers;
use crate::renderable::RenderableEnum;
use crate::renderable::ToRenderable;
use crate::surface::Surface;
use kintaro_egui_lib::InstanceMul;
use setup::Setup;

use crate::{
    clock::{Clock, RenderClock},
    config::Config,
    realtime::gui::GuiRepaintSignal,
};
use futures::executor::block_on;
use winit::window::Window;

use self::setup::Controls;

pub struct RealTimeState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub surface: Surface,
    pub clock: RenderClock,
    pub mouse_pressed: bool,

    pub composition: Composition,

    pub controls: Option<Controls>,
    pub base_instance_mul: InstanceMul,
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
        config: &Config<'static>,
        repaint_signal: std::sync::Arc<GuiRepaintSignal>,
        av_map: VisualsMap,
        audio_stream_handle: Option<rodio::Sink>,
    ) -> Result<RealTimeState, KintaroError> {
        let size = (config.window_size.0, config.window_size.1);
        println!("{}/{}", size.0, size.1);
        let Setup {
            device,
            surface,
            queue,
            controls,
            format,
        } = block_on(Setup::init(
            window,
            config,
            repaint_signal,
            audio_stream_handle,
        ))?;

        let (renderables, frame_names) =
            make_renderable_enums(&device, &queue, format, &av_map, config);

        let frames = make_frames(&device, size, format, frame_names)?;
        let base_instance_mul = controls.state.lock().unwrap().instance_mul.clone();

        Ok(Self {
            device,
            queue,
            size,
            clock: RenderClock::init(config),
            surface,
            // controls: Some(controls),
            controls: None,
            mouse_pressed: false,
            base_instance_mul,
            composition: Composition {
                renderables,
                camera: crate::camera::Camera::new(&config.cameras[0], size, 0),
                camera_configs: config.cameras.clone(),
                canvas: Canvas::init(size),
                frames,
            },
        })
    }

    pub fn play(&mut self) {
        self.clock.play();
        if let Some(ref mut controls) = self.controls {
            if let Some(a) = &controls.audio_stream_handle {
                a.play()
            }
        }
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.clock.pause();
        if let Some(ref mut controls) = self.controls {
            if let Some(a) = &controls.audio_stream_handle {
                a.pause()
            }
        }
    }
}

pub fn make_renderable_enums(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    av_map: &VisualsMap,
    config: &Config<'static>,
) -> (RenderableEnums, Vec<&'static str>) {
    let mut frame_names = vec![];
    let renderables = RenderableEnums(
        config
            .frame_passes
            .to_owned()
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
            .collect(),
    );

    (renderables, frame_names)
}
