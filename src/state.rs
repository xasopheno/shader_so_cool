use crate::{
    camera::{Camera, CameraController, Projection},
    clock::{Clock, RenderClock},
    config::Config,
    instance::{make_instances_and_instance_buffer, Instance},
    render_op::OpStream,
    render_pipleline::create_render_pipeline,
    setup::Setup,
    vertex::{create_index_buffer, create_vertex_buffer, Vertex},
};
use futures::executor::block_on;
use winit::window::Window;

pub struct RenderPassInput {
    pub vertex_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_bind_group: wgpu::BindGroup,
    pub index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub instances: Vec<Instance>,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub uniforms: crate::uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
}

pub struct State {
    pub clock: RenderClock,
    pub config: Config,
    pub renderpass: RenderPassInput,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub vertices: Vec<Vertex>,
    pub projection: Projection,
    pub camera_controller: CameraController,
    pub last_render_time: std::time::Instant,
    pub start_time: std::time::Instant,
    pub vertices_fn: fn() -> Vec<Vertex>,
    pub indices_fn: fn(u16) -> Vec<u16>,
    pub canvas: Canvas,
    pub clear_color: (f64, f64, f64),
    pub count: u32,
    pub camera: Camera,
    pub mouse_pressed: bool,
    pub op_stream: OpStream,
}

pub struct Canvas {
    pub ratio: f32,
    pub n_pixels: f32,
    pub n_row: u32,
    pub n_column: u32,
    pub instance_displacement: cgmath::Vector3<f32>,
}

pub fn canvas_info(size: (u32, u32)) -> Canvas {
    let ratio = size.0 as f32 / size.1 as f32;
    let n_pixels = 20.0;
    let n_row = (n_pixels * ratio) as u32;
    let n_column = n_pixels as u32;
    let instance_displacement: cgmath::Vector3<f32> =
        cgmath::Vector3::new(0.0, n_column as f32, n_pixels);

    Canvas {
        ratio,
        n_pixels,
        n_row,
        n_column,
        instance_displacement,
    }
}

impl State {
    pub fn init(window: &Window, config: &Config) -> State {
        let Setup {
            device,
            surface,
            queue,
            swap_chain,
            sc_desc,
            ..
        } = block_on(Setup::init(window, config));

        let op_stream = crate::render_op::OpStream::from_json(&config.filename);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let vertices = (config.vertices_fn)();
        let num_vertices = vertices.len() as u32;
        let indices = (config.indices_fn)(num_vertices as u16);
        let (instances, instance_buffer) = make_instances_and_instance_buffer(
            0,
            (window.inner_size().height, window.inner_size().height),
            &device,
        );
        let (camera, projection, camera_controller) =
            crate::camera::Camera::new((sc_desc.width, sc_desc.height), &config);
        let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            crate::uniforms::Uniforms::new(&device);
        let render_pipeline =
            create_render_pipeline(&device, &shader, &uniform_bind_group_layout, sc_desc.format);
        let vertex_buffer = create_vertex_buffer(&device, &vertices.as_slice());
        let index_buffer = create_index_buffer(&device, &indices.as_slice());
        let canvas = canvas_info((window.inner_size().height, window.inner_size().height));

        let renderpass_input = RenderPassInput {
            vertex_buffer,
            render_pipeline,
            uniform_bind_group,
            index_buffer,
            instance_buffer,
            instances,
            num_vertices,
            num_indices: indices.len() as u32,
            uniform_buffer,
            uniforms,
        };

        Self {
            clock: RenderClock::init(&config),
            renderpass: renderpass_input,
            count: 0,
            config: config.clone(),
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size: window.inner_size(),
            clear_color: crate::helpers::new_clear_color(),
            vertices: vertices.into(),
            mouse_pressed: false,
            camera,
            camera_controller,
            projection,
            last_render_time: std::time::Instant::now(),
            start_time: std::time::Instant::now(),
            vertices_fn: config.vertices_fn,
            indices_fn: config.indices_fn,
            canvas,
            op_stream,
        }
    }
}
