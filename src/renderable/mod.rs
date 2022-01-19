use kintaro_egui_lib::InstanceMul;

use crate::{
    canvas::Canvas, clock::ClockResult, glyphy::Glyphy, image_renderer::ImageRenderer,
    shared::RenderPassInput, toy::Toy, Config,
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
    fn to_renderable(&self) -> Result<RenderableEnum, wgpu::SurfaceError>;
}

pub enum RenderableEnum {
    Toy(Toy),
    ImageRenderer(ImageRenderer),
    Glyphy(Glyphy),
    EventStreams(Vec<RenderPassInput>),
}

pub enum RenderableConfig {
    Toy(ToyConfig),
    ImageRenderer(ImageRendererConfig),
    GlyphyConfig(GlyphyConfig),
    EventStreamConfig(EventStreamConfig),
}

pub struct ToyConfig {
    shader: wgpu::ShaderModule,
    size: (u32, u32),
    texture_format: wgpu::TextureFormat,
}

pub struct ImageRendererConfig {
    image_path: String,
    texture_format: wgpu::TextureFormat,
}

pub struct GlyphyConfig {
    text: String,
    texture_format: wgpu::TextureFormat,
}

pub struct EventStreamConfig {
    socool_filename: String,
    shader: wgpu::ShaderModule,
    texture_format: wgpu::TextureFormat,
}

pub struct RenderableConfigs(Vec<RenderableConfig>);
pub struct Renderables(Vec<RenderableEnum>);

impl ToRenderable for RenderableConfig {
    fn to_renderable(&self) -> Result<RenderableEnum, wgpu::SurfaceError> {
        todo!()
    }
}

impl RenderableConfigs {
    fn to_renderables(configs: RenderableConfigs) -> Result<Renderables, wgpu::SurfaceError> {
        Ok(Renderables(
            configs
                .0
                .iter()
                .map(|config| config.to_renderable().unwrap())
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
