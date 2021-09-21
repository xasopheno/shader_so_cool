mod input;
pub mod render;
mod resize;
mod setup;
mod update;

use setup::Setup;

use crate::{
    camera::Camera,
    canvas::Canvas,
    clock::{Clock, RenderClock},
    config::Config,
    instance::make_instances_and_instance_buffer,
    realtime::render::ExampleRepaintSignal,
    shared::{create_render_pipeline, helpers::new_clear_color, RenderPassInput},
    vertex::{create_index_buffer, create_vertex_buffer},
};
use futures::executor::block_on;
use winit::window::Window;

use self::setup::Gui;

pub struct RealTimeState {
    pub clock: RenderClock,
    pub config: Config,
    pub renderpasses: Vec<RenderPassInput>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub last_render_time: std::time::Instant,
    pub start_time: std::time::Instant,
    pub canvas: Canvas,
    pub clear_color: (f64, f64, f64),
    pub count: u32,
    pub camera: Camera,
    pub mouse_pressed: bool,
    pub gui: Gui,
    pub repaint_signal: std::sync::Arc<ExampleRepaintSignal>,
    pub audio_stream_handle: rodio::Sink,
}

impl RealTimeState {
    pub fn init(
        window: &Window,
        config: &Config,
        repaint_signal: std::sync::Arc<ExampleRepaintSignal>,
        audio_stream_handle: rodio::Sink,
    ) -> RealTimeState {
        let Setup {
            device,
            surface,
            queue,
            gui,
        } = block_on(Setup::init(window, config));

        let op_streams = crate::render_op::OpStream::from_json(&config.filename);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let renderpasses = op_streams
            .iter()
            .map(|op_stream| {
                let vertices = (config.vertices_fn)();
                let num_vertices = vertices.len() as u32;
                let indices = (config.indices_fn)(num_vertices as u16);
                let (instances, instance_buffer) = make_instances_and_instance_buffer(
                    0,
                    (window.inner_size().height, window.inner_size().height),
                    &device,
                );
                let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
                    crate::uniforms::Uniforms::new(&device);
                let render_pipeline = create_render_pipeline(
                    &device,
                    &shader,
                    &uniform_bind_group_layout,
                    wgpu::TextureFormat::Bgra8UnormSrgb,
                );
                RenderPassInput {
                    vertex_buffer: create_vertex_buffer(&device, &vertices.as_slice()),
                    index_buffer: create_index_buffer(&device, &indices.as_slice()),
                    vertices: vertices.into(),
                    op_stream: op_stream.to_owned(),
                    uniform_bind_group,
                    instances,
                    instance_buffer,
                    num_indices: indices.len() as u32,
                    uniform_buffer,
                    uniforms,
                    vertices_fn: config.vertices_fn,
                    indices_fn: config.indices_fn,
                    render_pipeline,
                }
            })
            .collect();
        let size = window.inner_size();

        Self {
            clock: RenderClock::init(&config),
            camera: crate::camera::Camera::new(
                &config.cameras[0],
                (size.width, size.height),
                &config,
            ),
            renderpasses,
            count: 0,
            config: config.clone(),
            size: (size.width.into(), size.height).into(),
            clear_color: new_clear_color(),
            mouse_pressed: false,
            last_render_time: std::time::Instant::now(),
            start_time: std::time::Instant::now(),
            surface,
            device,
            queue,
            canvas: Canvas::init((window.inner_size().height, window.inner_size().height)),
            gui,
            repaint_signal: repaint_signal.clone(),
            audio_stream_handle,
        }
    }

    pub fn play(&mut self) {
        self.clock.play();
        self.audio_stream_handle.play();
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        self.clock.pause();
        self.audio_stream_handle.pause();
    }
}
