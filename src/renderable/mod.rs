use std::collections::HashMap;

use kintaro_egui_lib::InstanceMul;
use winit::event::{ElementState, VirtualKeyCode};

use crate::{
    application::AvMap,
    canvas::Canvas,
    clock::ClockResult,
    config::{Config, FramePass},
    error::KintaroError,
    frame::types::Frame,
    glyphy::Glyphy,
    image_renderer::ImageRenderer,
    op_stream::renderpasses::make_renderpasses,
    origami::Origami,
    sampler::types::Sampler,
    shader::make_shader,
    shared::RenderPassInput,
    toy::Toy,
};

pub struct RenderableInput<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub clock_result: ClockResult,
    // pub view: &'a wgpu::TextureView,
    pub config: &'a Config<'a>,
    pub canvas: &'a Canvas,
    pub size: (u32, u32),
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
    pub instance_mul: InstanceMul,
    pub clear: bool,
    pub frames: &'a HashMap<String, Frame>,
}

pub trait Renderable<'a> {
    fn update(&mut self, input: &'a RenderableInput) -> Result<(), KintaroError>;
    fn render_pass(&mut self, input: &'a RenderableInput, clear: bool) -> Result<(), KintaroError>;
    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState);
}

pub trait ToRenderable {
    fn to_renderable(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &mut Config,
        av_map: &AvMap,
        format: wgpu::TextureFormat,
    ) -> Result<RenderableEnum, KintaroError>;
}

impl<'a> ToRenderable for RenderableConfig<'a> {
    fn to_renderable(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &mut Config,
        av_map: &AvMap,
        format: wgpu::TextureFormat,
    ) -> Result<RenderableEnum, KintaroError> {
        match self {
            RenderableConfig::Sampler(sampler_config) => {
                let _shader = make_shader(device, sampler_config.shader_path)?;
                let sampler = Sampler::new(device, config.window_size, format)?;
                Ok(RenderableEnum::Sampler(sampler))
            }
            RenderableConfig::Origami(origami_config) => {
                let shader = make_shader(device, origami_config.shader_path)?;
                let origami = Origami::init(device, format, shader, origami_config)?;
                Ok(RenderableEnum::Origami(origami))
            }
            RenderableConfig::Toy(renderable_config) => {
                let shader = make_shader(device, renderable_config.shader_path)?;
                let toy = crate::toy::setup_toy(device, shader, config.window_size, format);
                Ok(RenderableEnum::Toy(toy))
            }
            RenderableConfig::Glyphy(renderable_config) => {
                let glyphy = Glyphy::init(device, format, renderable_config.to_owned())
                    .expect("Unable to setup Glyphy");

                Ok(RenderableEnum::Glyphy(Box::new(glyphy)))
            }
            RenderableConfig::ImageRenderer(renderable_config) => {
                let image_renderer = pollster::block_on(ImageRenderer::new(
                    device,
                    queue,
                    format,
                    renderable_config.image_path,
                ))?;

                Ok(RenderableEnum::ImageRenderer(image_renderer))
            }
            RenderableConfig::EventStreams(renderable_config) => {
                let associated_av = av_map
                    .get(&renderable_config.socool_path)
                    .expect("No associated av in AvMap");
                let shader = make_shader(device, renderable_config.shader_path)?;
                let op_streams = crate::op_stream::OpStream::from_vec_op4d(associated_av);

                let renderpasses = make_renderpasses(device, op_streams, &shader, config, format);

                Ok(RenderableEnum::EventStreams(renderpasses))
            }
        }
    }
}

pub enum RenderableEnum {
    Toy(Toy),
    ImageRenderer(ImageRenderer),
    Glyphy(Box<Glyphy>),
    EventStreams(Vec<RenderPassInput>),
    Origami(Origami),
    Sampler(Sampler),
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

#[derive(Clone)]
pub struct ToyConfig<'a> {
    pub shader_path: &'a str,
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
    pub socool_path: String,
    pub shader_path: &'a str,
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
    fn update(&mut self, input: &'a RenderableInput) -> Result<(), KintaroError> {
        if let RenderableEnum::EventStreams(event_streams) = self {
            for renderpass in event_streams.iter_mut() {
                renderpass.update(
                    input.clock_result,
                    input.canvas,
                    input.device,
                    input.queue,
                    input.config,
                    input.size,
                    input.instance_mul,
                );
            }
        }

        Ok(())
    }

    fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) {
        if let RenderableEnum::Origami(origami) = self {
            origami.process_keyboard(key, state);
        }
    }

    fn render_pass(&mut self, input: &'a RenderableInput, clear: bool) -> Result<(), KintaroError> {
        match self {
            RenderableEnum::Sampler(sampler) => sampler.render(
                input.device,
                input.frames.get("frame1").unwrap(),
                input.frames.get("main").unwrap(),
            ),
            RenderableEnum::Origami(origami) => {
                origami.render(
                    input.device,
                    input.queue,
                    input.size,
                    &input.frames.get("frame1").unwrap().texture.view,
                    clear,
                );
            }
            RenderableEnum::EventStreams(event_streams) => {
                let mut encoder =
                    input
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("RenderPassInput Command Encoder"),
                        });

                for renderpass in event_streams.iter_mut() {
                    renderpass
                        .uniforms
                        .update_view_proj(input.view_position, input.view_proj);

                    renderpass.render(
                        &mut encoder,
                        &input.frames.get("frame1").unwrap().texture.view,
                        input.config,
                        !clear,
                    );
                }

                input.queue.submit(Some(encoder.finish()));
            }
            RenderableEnum::Toy(toy) => {
                toy.toy_renderpass(
                    input.clock_result.is_playing,
                    input.device,
                    input.queue,
                    &input.frames.get("frame1").unwrap().texture.view,
                    input.size,
                    input.clock_result.total_elapsed,
                    clear,
                )?;
            }
            RenderableEnum::Glyphy(glyphy) => {
                glyphy.render(
                    input.device,
                    input.queue,
                    input.size,
                    &input.frames.get("frame1").unwrap().texture.view,
                    clear,
                );
            }
            RenderableEnum::ImageRenderer(image_renderer) => {
                image_renderer.render(
                    input.device,
                    input.queue,
                    &input.frames.get("frame1").unwrap().texture.view,
                    clear,
                )?;
            }
        }

        Ok(())
    }
}
