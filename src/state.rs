use crate::{
    camera::{Camera, CameraController, Projection},
    texture,
    vertex::Vertex,
};
use image::GenericImageView;
use rand::prelude::*;
use rayon::prelude::*;
use wgpu::util::DeviceExt;
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
    // pub diffuse_bind_group: wgpu::BindGroup,
    // pub diffuse_texture: texture::Texture
}

fn random_color() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

impl State {
    pub fn new_random_clear_color() -> (f64, f64, f64) {
        (random_color(), random_color(), random_color())
    }

    pub fn new_shape() -> Vec<Vertex> {
        vec![
            Vertex::new(0.1, 0.1, 0.0),
            Vertex::new(-0.1, 0.1, 0.0),
            Vertex::new(-0.1, -0.1, 0.0),
            Vertex::new(0.1, -0.1, 0.0),
            Vertex::new(0.1, 0.1, 0.0),
            Vertex::new(-0.1, -0.1, 0.0),
        ]
    }

    pub fn new_random_vertices() -> Vec<Vertex> {
        (0..20)
            .into_par_iter()
            .map(|_| Vertex::new_random())
            .collect()
    }
    pub fn new_random_indices(n: u16) -> Vec<u16> {
        let mut rng = rand::thread_rng();
        let mut num = || rng.gen_range(0..n);

        (0..).map(|_| num()).collect()
    }

    pub fn new_shape_indices(n: u32) -> Vec<u32> {
        dbg!(n);
        // let mut indices: Vec<u32> = (0..n + 1).map(|i| i).collect();
        // indices.push(0);
        // indices
        vec![]
    }
    pub async fn new(window: &Window) -> Self {
        let vertices = State::new_shape();
        let size = window.inner_size();
        let num_vertices = vertices.len() as u32;
        let indices = State::new_shape_indices(num_vertices);

        // The instance is a handle to the GPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Unable to create adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Unable to request device.");

        let swap_chain_format = adapter
            .get_swap_chain_preferred_format(&surface)
            .expect("Unable to get preferred swap chain format");

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: swap_chain_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        // camera
        let camera =
            crate::camera::Camera::new((0.0, 0.0, 3.0), cgmath::Deg(-90.0), cgmath::Deg(0.0));
        let projection = crate::camera::Projection::new(
            sc_desc.width,
            sc_desc.height,
            cgmath::Deg(45.0),
            0.1,
            100.0,
        );

        let camera_controller = crate::camera::CameraController::new(4.0, 0.4);

        let mut uniforms = crate::uniforms::Uniforms::new();
        uniforms.update_view_proj(&camera, &projection);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &&uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        // let diffuse_bytes = include_bytes!("./milosh_2.png");
        // let diffuse_texture =
        // texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "milosh_2.png").unwrap();

        // let texture_bind_group_layout =
        // device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        // entries: &[
        // wgpu::BindGroupLayoutEntry {
        // binding: 0,
        // visibility: wgpu::ShaderStage::FRAGMENT,
        // ty: wgpu::BindingType::Texture {
        // multisampled: false,
        // view_dimension: wgpu::TextureViewDimension::D2,
        // sample_type: wgpu::TextureSampleType::Float { filterable: true },
        // },
        // count: None,
        // },
        // wgpu::BindGroupLayoutEntry {
        // binding: 1,
        // visibility: wgpu::ShaderStage::FRAGMENT,
        // ty: wgpu::BindingType::Sampler {
        // comparison: false,
        // filtering: true,
        // },
        // count: None,
        // },
        // ],
        // label: Some("texture_bind_group_layout"),
        // });

        // let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        // layout: &texture_bind_group_layout,
        // entries: &[
        // wgpu::BindGroupEntry {
        // binding: 0,
        // resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        // },
        // wgpu::BindGroupEntry {
        // binding: 1,
        // resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        // },
        // ],
        // label: Some("diffuse_bind_group"),
        // });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsage::INDEX,
        });

        let num_indices = indices.len() as u32;

        let clear_color = State::new_random_clear_color();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            clear_color,
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
            last_render_time: std::time::Instant::now()
            // diffuse_bind_group,
            // diffuse_texture,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
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
            DeviceEvent::Button {
                button: 0, // Left Mouse Button
                state,
            } => {
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
        if self.count > 500 {
            // self.vertices = State::new_random_vertices();
            // self.clear_color = State::new_random_clear_color();
            self.count = 0;
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

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices.as_slice()),
                usage: wgpu::BufferUsage::VERTEX,
            });

        self.vertex_buffer = vertex_buffer;
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
            // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}
