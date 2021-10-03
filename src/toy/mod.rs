mod shader;
mod uniforms;

pub use shader::*;

use crate::shared::create_render_pipeline;

use self::uniforms::ToyUniforms;

pub struct Toy {
    pub start_time: std::time::Instant,
    pub shader: wgpu::ShaderModule,
    pub uniforms: ToyUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub size: (u32, u32),
}

pub fn setup_toy(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface: &wgpu::Surface,
    start_time: std::time::Instant,
    size: (u32, u32),
) -> Toy {
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./toy.wgsl").into()),
    });

    let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        uniforms::ToyUniforms::new(device);

    let render_pipeline = create_render_pipeline(
        device,
        &shader,
        &uniform_bind_group_layout,
        wgpu::TextureFormat::Bgra8UnormSrgb,
    );

    Toy {
        start_time,
        size,
        shader,
        render_pipeline,
        uniforms,
        uniform_buffer,
        uniform_bind_group,
    }
}

fn toy_renderpass(
    toy: Toy,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    surface: &wgpu::Surface,
) -> Result<(), wgpu::SurfaceError> {
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        queue.write_buffer(
            &toy.uniform_buffer,
            0,
            bytemuck::cast_slice(&[toy.uniforms]),
        );

        let clear_color = wgpu::Color {
            r: 0.2,
            g: 0.2,
            b: 0.25,
            a: 1.0,
        };

        let output = surface.get_current_frame()?.output;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        rpass.set_pipeline(&toy.render_pipeline);
        rpass.set_bind_group(0, &toy.uniform_bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    Ok(())
}
