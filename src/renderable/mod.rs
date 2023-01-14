use std::collections::HashMap;

use kintaro_egui_lib::InstanceMul;
use winit::event::{ElementState, VirtualKeyCode};

use crate::{
    canvas::Canvas,
    clock::ClockResult,
    error::KintaroError,
    frame::types::Frame,
    glyphy::Glyphy,
    image_renderer::ImageRenderer,
    op_stream::renderpasses::make_renderpass,
    op_stream::{GetOps, OpReceiver},
    origami::Origami,
    sampler::types::Sampler,
    shader::make_shader,
    shared::EventStreams,
    toy::Toy,
    vertex::shape::Shape,
    Instancer,
};

pub struct RenderableInput<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub clock_result: ClockResult,
    pub canvas: &'a Canvas,
    pub size: (u32, u32),
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
    pub instance_mul: InstanceMul,
    pub clear: bool,
    pub frames: &'a HashMap<String, Frame>,
}

pub trait Renderable<'a> {
    fn update(
        &mut self,
        input: &'a RenderableInput,
        receiver: &mut OpReceiver,
    ) -> Result<(), KintaroError>;
    fn render_pass(&mut self, input: &'a RenderableInput, clear: bool) -> Result<(), KintaroError>;
    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState);
}

pub trait ToRenderable {
    fn to_renderable(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window_size: (u32, u32),
        format: wgpu::TextureFormat,
        output_frame: String,
    ) -> Result<RenderableEnum, KintaroError>;
    fn watchable_paths(&self) -> Vec<String>;
}

impl<'a> ToRenderable for RenderableConfig<'a> {
    fn to_renderable(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window_size: (u32, u32),
        format: wgpu::TextureFormat,
        output_frame: String,
    ) -> Result<RenderableEnum, KintaroError> {
        match self {
            RenderableConfig::Sampler(sampler_config) => {
                let _shader = make_shader(device, sampler_config.shader_path)?;
                let sampler = Sampler::new(
                    device,
                    window_size,
                    format,
                    sampler_config.input_frame.to_string(),
                )?;
                Ok(RenderableEnum::Sampler(output_frame, sampler))
            }
            RenderableConfig::Origami(origami_config) => {
                let shader = make_shader(device, origami_config.shader_path)?;
                let origami = Origami::init(device, format, shader, origami_config)?;
                Ok(RenderableEnum::Origami(output_frame, origami))
            }
            RenderableConfig::Toy(renderable_config) => {
                let shader = make_shader(device, renderable_config.shader_path)?;
                let toy = crate::toy::setup_toy(
                    device,
                    shader,
                    window_size,
                    format,
                    // renderable_config.rgba,
                );
                Ok(RenderableEnum::Toy(output_frame, toy))
            }
            RenderableConfig::Glyphy(renderable_config) => {
                let glyphy = Glyphy::init(device, format, renderable_config.to_owned())
                    .expect("Unable to setup Glyphy");

                Ok(RenderableEnum::Glyphy(output_frame, Box::new(glyphy)))
            }
            RenderableConfig::ImageRenderer(renderable_config) => {
                let image_renderer = pollster::block_on(ImageRenderer::new(
                    device,
                    queue,
                    format,
                    renderable_config.image_path,
                ))?;

                Ok(RenderableEnum::ImageRenderer(output_frame, image_renderer))
            }
            RenderableConfig::EventStreams(renderable_config) => {
                let shader = make_shader(device, renderable_config.shader_path)?;
                let names = renderable_config.shape.color.names();
                let event_streams = names
                    .into_iter()
                    .map(|name| {
                        let renderpass = make_renderpass(
                            device,
                            &shader,
                            window_size,
                            format,
                            renderable_config.shape.to_owned(),
                            renderable_config.instancer.to_owned(),
                            name.clone(),
                        );

                        (name, renderpass)
                    })
                    .collect::<std::collections::HashMap<String, _>>();

                Ok(RenderableEnum::EventStreams(output_frame, event_streams))
            }
        }
    }
    fn watchable_paths(&self) -> Vec<String> {
        match self {
            RenderableConfig::Toy(config) => {
                vec![config.shader_path.to_string()]
            }
            RenderableConfig::EventStreams(config) => {
                vec![config.shader_path.to_string()]
            }
            _ => {
                vec![]
            }
        }
    }
}

pub enum RenderableEnum {
    Toy(String, Toy),
    ImageRenderer(String, ImageRenderer),
    Glyphy(String, Box<Glyphy>),
    EventStreams(String, EventStreams),
    Origami(String, Origami),
    Sampler(String, Sampler),
}

#[derive(Clone)]
pub enum RenderableConfig<'a> {
    Toy(ToyConfig<'a>),
    ImageRenderer(ImageRendererConfig<'a>),
    Glyphy(GlyphyConfig),
    EventStreams(EventStreamConfig<'a>),
    Origami(OrigamiConfig<'a>),
    Sampler(SamplerConfig<'a>),
}

#[derive(Clone, Debug, Copy)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Clone, Debug)]
pub struct ToyConfig<'a> {
    pub shader_path: &'a str,
    // pub rgba: RGBA,
}

#[derive(Clone)]
pub struct SamplerConfig<'a> {
    pub shader_path: &'a str,
    pub input_frame: &'a str,
}

#[derive(Clone)]
pub struct ImageRendererConfig<'a> {
    pub image_path: &'a str,
}

#[derive(Clone)]
pub struct OrigamiConfig<'a> {
    pub shader_path: &'a str,
    pub n_vertices: usize,
    pub n_indices: usize,
}

#[derive(Clone)]
pub struct EventStreamConfig<'a> {
    pub shader_path: &'a str,
    pub shape: Shape,
    pub instancer: Box<dyn Instancer>,
}

#[derive(Clone)]
pub enum GlyphyConfig {
    GlyphyNamedColorSetConfig {
        text: Vec<(&'static str, Vec<&'static str>)>,
        location: (f32, f32),
        scale: f32,
    },
    GlypyTextConfig {
        text: Vec<(&'static str, &'static str)>,
        location: (f32, f32),
        scale: f32,
    },
}

impl<'a> Renderable<'a> for RenderableEnum {
    fn update(
        &mut self,
        input: &'a RenderableInput,
        receiver: &mut OpReceiver,
    ) -> Result<(), KintaroError> {
        if let RenderableEnum::EventStreams(_, event_streams) = self {
            for (name, renderpass) in event_streams.iter_mut() {
                let ops = receiver.get_batch(input.clock_result.total_elapsed, name);
                renderpass.update(
                    input.clock_result,
                    input.canvas,
                    input.device,
                    input.queue,
                    input.size,
                    input.instance_mul,
                    &ops,
                );
            }
        }

        Ok(())
    }

    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) {
        if let RenderableEnum::Origami(_, origami) = self {
            origami.process_keyboard(key, state);
        }
    }

    fn render_pass(&mut self, input: &'a RenderableInput, clear: bool) -> Result<(), KintaroError> {
        match self {
            RenderableEnum::Sampler(output_frame, sampler) => sampler.render(
                input.device,
                input.queue,
                input.frames.get(&sampler.input_frame).unwrap(),
                input.frames.get(output_frame).unwrap(),
            ),
            RenderableEnum::Origami(output_frame, origami) => {
                origami.render(
                    input.device,
                    input.queue,
                    input.size,
                    &input.frames.get(output_frame).unwrap().texture.view,
                    clear,
                );
            }
            RenderableEnum::EventStreams(output_frame, event_streams) => {
                let mut encoder =
                    input
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("RenderPassInput Command Encoder"),
                        });

                for (_, renderpass) in event_streams.iter_mut() {
                    renderpass
                        .uniforms
                        .update_view_proj(input.view_position, input.view_proj);

                    renderpass.render(
                        &mut encoder,
                        &input.frames.get(output_frame).unwrap().texture.view,
                        !clear,
                    );
                }

                input.queue.submit(Some(encoder.finish()));
            }
            RenderableEnum::Toy(output_frame, toy) => {
                toy.toy_renderpass(
                    input.clock_result.is_playing,
                    input.device,
                    input.queue,
                    &input.frames.get(output_frame).unwrap().texture.view,
                    input.size,
                    input.clock_result.total_elapsed,
                    clear,
                )?;
            }
            RenderableEnum::Glyphy(output_frame, glyphy) => {
                glyphy.render(
                    input.device,
                    input.queue,
                    input.size,
                    &input.frames.get(output_frame).unwrap().texture.view,
                    clear,
                );
            }
            RenderableEnum::ImageRenderer(output_frame, image_renderer) => {
                image_renderer.render(
                    input.device,
                    input.queue,
                    &input.frames.get(output_frame).unwrap().texture.view,
                    clear,
                )?;
            }
        }

        Ok(())
    }
}
