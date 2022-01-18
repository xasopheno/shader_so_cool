use kintaro_egui_lib::InstanceMul;

use crate::{
    canvas::Canvas, clock::ClockResult, image_renderer::ImageRenderer, shared::RenderPassInput,
    toy::Toy, Config,
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
    fn update(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError>;
    fn render_pass(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError>;
}

impl<'a> Renderable<'a> for Toy {
    fn render_pass(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        self.toy_renderpass(
            input.clock_result.is_playing,
            input.device,
            input.queue,
            input.view,
            input.size,
            input.clock_result.total_elapsed,
            input.clear,
        )
    }
    fn update(&mut self, _input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
}

impl<'a> Renderable<'a> for ImageRenderer {
    fn render_pass(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        self.render(input.device, input.queue, input.view)
    }

    fn update(&mut self, _input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }
}

impl<'a> Renderable<'a> for Vec<RenderPassInput> {
    fn update(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        for (idx, renderpass) in self.iter_mut().enumerate() {
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

        Ok(())
    }

    fn render_pass(&mut self, input: &'a RenderableInput) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = input
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("RenderPassInput Command Encoder"),
            });

        for renderpass in self.iter_mut() {
            renderpass
                .uniforms
                .update_view_proj(input.view_position, input.view_proj);

            renderpass.render(&mut encoder, input.view, input.config, !input.clear);
        }

        input.queue.submit(Some(encoder.finish()));

        Ok(())
    }
}
