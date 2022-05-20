pub mod gui;
mod input;
pub mod render;
mod resize;
pub mod setup;

use crate::{
    application::VisualsMap,
    camera::Cameras,
    canvas::Canvas,
    clock::{Clock, RenderClock},
    composition::Composition,
    composition::RenderableEnums,
    config::Config,
    error::KintaroError,
    frame::types::Frame,
    frame::types::Frames,
    frame::vertex::make_square_buffers,
    realtime::gui::GuiRepaintSignal,
    renderable::RenderableEnum,
    renderable::ToRenderable,
    surface::Surface,
};
use futures::executor::block_on;
use kintaro_egui_lib::InstanceMul;
use setup::Setup;
use std::collections::HashMap;
use winit::window::Window;

use self::setup::Controls;

pub struct RealTimeState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: (u32, u32),
    pub surface: Surface,
    pub canvas: Canvas,
    pub clock: RenderClock,
    pub mouse_pressed: bool,
    pub base_instance_mul: InstanceMul,

    pub controls: Option<Controls>,
    pub cameras: Cameras,

    pub composition: Option<Composition>,
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
        repaint_signal: Option<std::sync::Arc<GuiRepaintSignal>>,
    ) -> Result<RealTimeState, KintaroError> {
        let size = (config.window_size.0, config.window_size.1);
        println!("{}/{}", size.0, size.1);
        let Setup {
            device,
            surface,
            queue,
            controls,
            format,
        } = block_on(Setup::init(window, config, repaint_signal))?;

        let base_instance_mul = config.instance_mul;

        let composition = Composition::init_realtime(&device, &queue, format, config)?;

        Ok(Self {
            device,
            queue,
            size,
            surface,

            clock: RenderClock::init(config),
            canvas: Canvas::init(size),

            controls,
            mouse_pressed: false,

            base_instance_mul,
            cameras: Cameras {
                current: crate::camera::Camera::new(&config.cameras[0], size),
                configs: config.cameras.clone(),
                index: 0,
            },

            composition: Some(composition),
        })
    }

    pub fn push_composition(&mut self, config: &Config<'static>) -> Result<(), KintaroError> {
        self.pause();
        self.composition = Some(Composition::init_realtime(
            &self.device,
            &self.queue,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            config,
        )?);
        self.play();

        Ok(())
    }

    pub fn play(&mut self) {
        self.clock.play();
        if let Some(ref mut composition) = self.composition {
            if let Some(a) = &composition.audio_stream_handle {
                a.play()
            }
        }
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.clock.pause();
        if let Some(ref mut composition) = self.composition {
            if let Some(a) = &composition.audio_stream_handle {
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
                                config.window_size,
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
