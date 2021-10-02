use winit::{
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
mod shader;

pub use shader::*;

unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

#[derive(Copy, Clone)]
#[allow(unused_attributes)]
// #[spirv(block)]
pub struct ShaderConstants {
    pub width: f32,
    pub height: f32,
    pub frame: f32,
    pub time: f32,
}

async fn setup(setup: crate::realtime::setup::Setup, size: (u32, u32)) {
    let shader = setup
        .device
        .create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./toy.wgsl").into()),
        });
}
fn render_pass(
    setup: crate::realtime::setup::Setup,
    size: (u32, u32),
) -> Result<(), wgpu::SurfaceError> {
    let start = std::time::Instant::now();

    let mut frame_count = 0.0;

    let output = setup.surface.get_current_frame()?.output;
    let mut encoder = setup
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let clear_color = wgpu::Color {
            r: 0.2,
            g: 0.2,
            b: 0.25,
            a: 1.0,
        };

        let output = setup.surface.get_current_frame()?.output;
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
        let push_constants = ShaderConstants {
            width: size.0 as f32,
            height: size.1 as f32,
            frame: frame_count,
            time: start.elapsed().as_secs_f32(),
        };
        rpass.set_pipeline(shader.pipeline());
        rpass.set_push_constants(wgpu::ShaderStages::all(), 0, unsafe {
            as_u8_slice(&push_constants)
        });
        rpass.draw(0..3, 0..1);

        frame_count += 1.0;
    }

    setup.queue.submit(Some(encoder.finish()));
    Ok(())
}
