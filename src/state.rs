use crate::{
    camera::{Camera, CameraController, Projection},
    instance::{
        make_instance_buffer, make_instances, make_instances_and_instance_buffer, Instance,
    },
    render_op::Op4D,
    render_pipleline::create_render_pipeline,
    setup::Setup,
    vertex::{create_index_buffer, create_vertex_buffer, make_vertex_buffer, Vertex},
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
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
    pub vertices_fn: fn() -> Vec<Vertex>,
    pub indices_fn: fn(u16) -> Vec<u16>,
    pub canvas: Canvas,
}

pub struct Canvas {
    ratio: f32,
    n_pixels: f32,
    n_row: u32,
    n_column: u32,
    instance_displacement: cgmath::Vector3<f32>,
}

fn canvas_info(size: winit::dpi::PhysicalSize<u32>) -> Canvas {
    let ratio = size.width as f32 / size.height as f32;
    let n_pixels = 20.0;
    let n_row = (n_pixels * ratio) as u32;
    let n_column = n_pixels as u32;
    let instance_displacement: cgmath::Vector3<f32> =
        cgmath::Vector3::new(n_row as f32 - 1.0, (n_column - 1) as f32, n_pixels * 2.7);

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
        (0..50)
            .into_par_iter()
            .map(|_| Vertex::new_random())
            .collect()
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
            instances,
            instance_buffer,
            vertices_fn,
            indices_fn,
            canvas,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.canvas = canvas_info(new_size);

        let (instances, instance_buffer) =
            make_instances_and_instance_buffer(100, new_size, &self.device);
        self.instances = instances;
        self.instance_buffer = instance_buffer;

        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
        self.projection.resize(new_size.width, new_size.height);
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

    pub fn update(&mut self, dt: std::time::Duration) {
        self.count += 1;
        if self.count % 400 == 0 {
            self.vertices = (self.vertices_fn)();
            self.clear_color = State::new_random_clear_color();
        }
        // self.vertices.par_iter_mut().for_each(|v| v.update());
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.uniforms
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let now = std::time::Instant::now();
        let dt = now - self.last_render_time;
        self.last_render_time = now;
        self.update(dt);
        let frame = self.swap_chain.get_current_frame().unwrap().output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.vertex_buffer = make_vertex_buffer(&self.device, self.vertices.as_slice());
        let mut new_instances: Vec<Instance> = Op4D::vec_random(7)
            .into_par_iter()
            .map(|op| {
                op.into_instance(
                    &self.canvas.instance_displacement,
                    self.canvas.n_column,
                    self.canvas.n_row,
                )
            })
            .collect();

        // let instance_displacement: cgmath::Vector3<f32> =
        // cgmath::Vector3::new(n_row as f32 - 1.0, (n_column - 1) as f32, n_pixels * 2.7);

        self.instances.append(&mut new_instances);
        self.instances.par_iter_mut().for_each(|i| {
            i.update_state(dt.as_secs_f32() as f32);
        });

        self.instances.retain(|i| i.life > 0.0);
        self.instance_buffer = make_instance_buffer(&self.instances, self.size, &self.device);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // This is what [[location(0)]] in the fragment shader targets
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.clear_color.0,
                            g: self.clear_color.1,
                            b: self.clear_color.2,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
