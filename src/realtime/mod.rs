pub mod gui;
mod input;
pub mod render;
mod resize;
pub mod setup;

use crate::{
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
    op_stream::{GetOps, OpReceiver},
    realtime::gui::GuiRepaintSignal,
    renderable::RenderableEnum,
    renderable::ToRenderable,
    surface::Surface,
};
use futures::executor::block_on;
use kintaro_egui_lib::InstanceMul;
use setup::Setup;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
pub use weresocool::{
    generation::Op4D,
    manager::{prepare_render_outside, VisEvent},
    {interpretable::InputType::Filename, manager::RenderManager},
};
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

    pub receiver: OpReceiver,
    pub render_manager: Arc<Mutex<RenderManager>>,
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
        receiver: Option<crossbeam_channel::Receiver<VisEvent>>,
        render_manager: Arc<Mutex<RenderManager>>,
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
        let input = if let Some(channel) = receiver {
            channel
        } else {
            unimplemented!()
        };

        Ok(Self {
            device,
            queue,
            size,
            surface,

            clock: RenderClock::init(),
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
            receiver: OpReceiver::init(None, Some(input)),
            render_manager,
        })
    }

    pub fn push_composition(&mut self, config: &Config<'static>) -> Result<(), KintaroError> {
        let render_voices = match prepare_render_outside(Filename(config.socool_path), None) {
            Ok(result) => Some(result),
            Err(error) => {
                println!("{}", error);
                None
            }
        };

        if let Some(voices) = render_voices {
            self.render_manager.lock().unwrap().push_render(voices);
        }
        let composition = Composition::init_realtime(
            &self.device,
            &self.queue,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            config,
        )?;
        self.composition = Some(composition);
        self.clock.reset();

        Ok(())
    }

    pub fn play(&mut self) {
        self.render_manager.lock().unwrap().play();
        self.clock.play();
    }

    pub fn pause(&mut self) {
        self.render_manager.lock().unwrap().pause();
        self.clock.pause();
    }

    pub fn is_paused(&self) -> bool {
        !self.clock.is_playing()
    }
}

pub fn make_renderable_enums(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    format: wgpu::TextureFormat,
    config: &Config<'static>,
) -> Result<(RenderableEnums, Vec<&'static str>), KintaroError> {
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
                        renderable_config.to_renderable(
                            &device,
                            &queue,
                            config.window_size,
                            format,
                            frame_pass.output_frame.to_string(),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .into_iter()
            .flatten()
            .collect::<Vec<RenderableEnum>>(),
    );

    Ok((renderables, frame_names))
}
