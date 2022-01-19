use kintaro_egui_lib::InstanceMul;

use crate::{
    canvas::Canvas, clock::ClockResult, config::Config, glyphy::Glyphy,
    image_renderer::ImageRenderer, op_stream::renderpasses::make_renderpasses, shader::make_shader,
    shared::RenderPassInput, toy::Toy,
};

pub struct RenderableInput<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub clock_result: ClockResult,
    pub view: &'a wgpu::TextureView,
    pub config: &'a Config<'a>,
    pub canvas: &'a Canvas,
    pub size: (u32, u32),
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
    pub instance_mul: InstanceMul,
    pub clear: bool,
}

pub trait Renderable<'a> {
    // TODO: Fix error types
    fn update(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError>;
    fn render_pass(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError>;
}

pub trait ToRenderable {
    fn to_renderable(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &Config,
    ) -> Result<RenderableEnum, wgpu::SurfaceError>;
}

impl<'a> ToRenderable for RenderableConfig<'a> {
    fn to_renderable(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &Config,
    ) -> Result<RenderableEnum, wgpu::SurfaceError> {
        match self {
            RenderableConfig::Toy(renderable_config) => {
                let shader = make_shader(&device, &renderable_config.shader_path).unwrap();
                let toy = crate::toy::setup_toy(
                    device,
                    shader,
                    renderable_config.size,
                    renderable_config.texture_format,
                );
                Ok(RenderableEnum::Toy(toy))
            }
            RenderableConfig::Glyphy(renderable_config) => {
                let glyphy = Glyphy::init(
                    &device,
                    wgpu::TextureFormat::Bgra8UnormSrgb,
                    renderable_config.text.to_vec(),
                )
                .expect("Unable to setup Glyphy");

                Ok(RenderableEnum::Glyphy(glyphy))
            }
            RenderableConfig::ImageRenderer(renderable_config) => {
                // TODO need to pass in image
                let image_renderer = pollster::block_on(ImageRenderer::new(
                    device,
                    &queue,
                    renderable_config.texture_format,
                ));

                Ok(RenderableEnum::ImageRenderer(image_renderer))
            }
            RenderableConfig::EventStreams(renderable_config) => {
                let shader = make_shader(&device, &renderable_config.shader_path).unwrap();
                // let op_streams = crate::op_stream::OpStream::from_vec_op4d(av);

                // let renderpasses = make_renderpasses(
                // &device,
                // op_streams,
                // &instance_shader,
                // config,
                // wgpu::TextureFormat::Bgra8UnormSrgb,
                // );

                todo!()
            }
        }
    }
}

pub enum RenderableEnum {
    Toy(Toy),
    ImageRenderer(ImageRenderer),
    Glyphy(Glyphy),
    EventStreams(Vec<RenderPassInput>),
}

pub enum RenderableConfig<'a> {
    Toy(ToyConfig<'a>),
    ImageRenderer(ImageRendererConfig<'a>),
    Glyphy(GlyphyConfig),
    EventStreams(EventStreamConfig<'a>),
}

pub struct ToyConfig<'a> {
    shader_path: &'a str,

    size: (u32, u32),
    texture_format: wgpu::TextureFormat,
}

pub struct ImageRendererConfig<'a> {
    image_path: &'a str,
    texture_format: wgpu::TextureFormat,
}

pub struct GlyphyConfig {
    text: Vec<(&'static str, Vec<&'static str>)>,
    texture_format: wgpu::TextureFormat,
}

pub struct EventStreamConfig<'a> {
    socool_path: String,
    shader_path: &'a str,
    texture_format: wgpu::TextureFormat,
}

pub struct RenderableConfigs<'a>(Vec<RenderableConfig<'a>>);
pub struct Renderables(Vec<RenderableEnum>);

impl<'a> RenderableConfigs<'a> {
    fn to_renderables(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        config: &Config,
        renderable_configs: RenderableConfigs,
    ) -> Result<Renderables, wgpu::SurfaceError> {
        Ok(Renderables(
            renderable_configs
                .0
                .iter()
                .map(|renderable_config| {
                    renderable_config
                        .to_renderable(device, queue, config)
                        .unwrap()
                })
                .collect(),
        ))
    }
}

impl<'a> Renderable<'a> for RenderableEnum {
    fn update(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        match self {
            RenderableEnum::EventStreams(event_streams) => {
                for (idx, renderpass) in event_streams.iter_mut().enumerate() {
                    renderpass.update(
                        idx,
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
            _ => {}
        }

        Ok(())
    }
    fn render_pass(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        match self {
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

                    renderpass.render(&mut encoder, input.view, input.config, !input.clear);
                }

                input.queue.submit(Some(encoder.finish()));
            }
            RenderableEnum::Toy(toy) => {
                toy.toy_renderpass(
                    input.clock_result.is_playing,
                    input.device,
                    input.queue,
                    input.view,
                    input.size,
                    input.clock_result.total_elapsed,
                    input.clear,
                )?;
            }
            RenderableEnum::Glyphy(glyphy) => {
                glyphy.render(input.device, input.queue, input.size, input.view, false);
            }
            RenderableEnum::ImageRenderer(image_renderer) => {
                image_renderer.render(input.device, input.queue, input.view)?;
            }
        }

        Ok(())
    }
}
