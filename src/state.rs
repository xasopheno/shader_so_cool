use crate::{
    camera::{Camera, CameraController, Projection},
    instance::{make_instances_and_instance_buffer, Instance},
    render_op::{OpStream, ToInstance},
    render_pipleline::create_render_pipeline,
    setup::Setup,
    vertex::{create_index_buffer, create_vertex_buffer, Vertex},
};
use rand::prelude::*;
use rayon::prelude::*;
use winit::{event::*, window::Window};

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub swap_chain_format: wgpu::TextureFormat,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub clear_color: (f64, f64, f64),
    pub vertices: Vec<Vertex>,
    pub count: u32,
    pub camera: Camera,
    pub uniforms: crate::uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub projection: Projection,
    pub camera_controller: CameraController,
    pub mouse_pressed: bool,
    pub last_render_time: std::time::Instant,
    pub start_time: std::time::Instant,
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
    pub vertices_fn: fn() -> Vec<Vertex>,
    pub indices_fn: fn(u16) -> Vec<u16>,
    pub canvas: Canvas,
    pub op_stream: OpStream,
}

pub struct Canvas {
    pub ratio: f32,
    pub n_pixels: f32,
    pub n_row: u32,
    pub n_column: u32,
    pub instance_displacement: cgmath::Vector3<f32>,
}

pub fn canvas_info(size: winit::dpi::PhysicalSize<u32>) -> Canvas {
    let ratio = size.width as f32 / size.height as f32;
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

#[allow(dead_code)]
fn random_color() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

impl State {
    pub fn new_clear_color() -> (f64, f64, f64) {
        (0.7, 0.3, 0.6)
    }

    #[allow(dead_code)]
    pub fn new_random_clear_color() -> (f64, f64, f64) {
        (random_color(), random_color(), random_color())
    }

    #[allow(dead_code)]
    pub fn new_random_vertices() -> Vec<Vertex> {
        (0..50).into_iter().map(|_| Vertex::new_random()).collect()
    }

    #[allow(dead_code)]
    pub fn new_random_indices(n: u16) -> Vec<u16> {
        let mut rng = rand::thread_rng();
        let mut num = || rng.gen_range(0..n);

        (0..50).map(|_| num()).collect()
    }

    #[allow(dead_code)]
    pub fn new_shape_vertices() -> Vec<Vertex> {
        let size = 1.0;
        vec![
            Vertex::new(size, size, 0.0),
            Vertex::new(-size, size, 0.0),
            Vertex::new(-size, -size, 0.0),
            Vertex::new(size, -size, 0.0),
        ]
    }

    #[allow(dead_code)]
    pub fn new_shape_indices(_n: u16) -> Vec<u16> {
        vec![0, 1, 2, 0, 2, 3]
    }

    pub async fn new(window: &Window) -> Self {
        let Setup {
            device,
            surface,
            queue,
            swap_chain,
            swap_chain_format,
            sc_desc,
            ..
        } = Setup::init(window).await;

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let vertices_fn = State::new_random_vertices;
        let indices_fn = State::new_random_indices;

        let vertices = vertices_fn();
        let num_vertices = vertices.len() as u32;
        let indices = indices_fn(num_vertices as u16);
        let (instances, instance_buffer) =
            make_instances_and_instance_buffer(0, window.inner_size(), &device);
        let (camera, projection, camera_controller) = crate::camera::Camera::new(&sc_desc);
        let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
            crate::uniforms::Uniforms::new(&device);
        let render_pipeline =
            create_render_pipeline(&device, &shader, &uniform_bind_group_layout, &sc_desc);
        let vertex_buffer = create_vertex_buffer(&device, &vertices.as_slice());
        let index_buffer = create_index_buffer(&device, &indices.as_slice());
        let canvas = canvas_info(window.inner_size());
        // let ops = Op4D::vec_random(1000);

        let op_stream = OpStream::from_json();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size: window.inner_size(),
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices: indices.len() as u32,
            clear_color: State::new_clear_color(),
            vertices: vertices.into(),
            count: 0,
            swap_chain_format,
            mouse_pressed: false,
            camera,
            camera_controller,
            projection,
            uniform_bind_group,
            uniform_buffer,
            uniforms,
            last_render_time: std::time::Instant::now(),
            start_time: std::time::Instant::now(),
            instances,
            instance_buffer,
            vertices_fn,
            indices_fn,
            canvas,
            op_stream,
        }
    }

    pub fn keyboard_input(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                } => {
                    self.camera_controller.process_keyboard(*key, *state);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
            }) => self.camera_controller.process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(&*delta);
                true
            }
            DeviceEvent::Button { button: _, state } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera_controller.process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }
}
