mod create_toy_render_pipeline;
mod uniforms;

use self::uniforms::ToyUniforms;

pub struct Toy {
    pub shader: wgpu::ShaderModule,
    pub uniforms: ToyUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub size: (u32, u32),
}

pub fn setup_toy(device: &wgpu::Device, size: (u32, u32), format: wgpu::TextureFormat) -> Toy {
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Toy Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../toy.wgsl").into()),
    });

    let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        uniforms::ToyUniforms::new(device);

    let render_pipeline = create_toy_render_pipeline::create_toy_render_pipeline(
        device,
        &shader,
        &uniform_bind_group_layout,
        format,
    );

    Toy {
        size,
        shader,
        render_pipeline,
        uniforms,
        uniform_buffer,
        uniform_bind_group,
    }
}

pub fn toy_renderpass(
    is_playing: bool,
    toy: &mut Toy,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    view: &wgpu::TextureView,
    size: (u32, u32),
    total_elapsed: f32,
) -> Result<(), wgpu::SurfaceError> {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    // if is_playing {
    toy.uniforms
        .update_uniforms((size.0, size.1), total_elapsed);
    queue.write_buffer(
        &toy.uniform_buffer,
        0,
        bytemuck::cast_slice(&[toy.uniforms]),
    );
    // }
    {
        let clear_color = wgpu::Color {
            r: 0.2,
            g: 0.2,
            b: 0.25,
            a: 1.0,
        };

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
