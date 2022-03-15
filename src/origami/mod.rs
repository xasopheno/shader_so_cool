mod render_pipeline;
mod uniforms;
mod vertex;
use rand::Rng;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, VirtualKeyCode};

use crate::error::KintaroError;
use crate::renderable::OrigamiConfig;
// use crate::shared::helpers::new_clear_color;
// use crate::shared::new_random_clear_color;

use self::render_pipeline::create_origami_render_pipeline;
use self::vertex::OrigamiVertex;

pub struct Origami {
    pub vertices: Vec<OrigamiVertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub num_vertices: u32,
    pub uniforms: crate::origami::uniforms::OrigamiUniforms,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
}

fn random_color() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

impl Origami {
    pub fn new_random_clear_color() -> (f64, f64, f64) {
        (random_color(), random_color(), random_color())
    }
    pub fn new_random_vertices(n: usize) -> Vec<OrigamiVertex> {
        (0..n)
            .into_iter()
            .map(|_| OrigamiVertex::new_random())
            .collect()
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) {
        if key == VirtualKeyCode::N && state == ElementState::Pressed {
            self.vertices = Self::new_random_vertices(20);
            // dbg!(&self.vertices);
        }
        if key == VirtualKeyCode::P && state == ElementState::Released {
            println!("Printing frame");
            // self.vertices = Self::new_random_vertices(20);
            // dbg!(&self.vertices);
        }
    }

    pub fn new_random_indices(n_indices: u32, n_generate: u32) -> Vec<u32> {
        let mut rng = rand::thread_rng();
        let mut num = || rng.gen_range(0..n_indices);

        (0..n_generate).map(|_| num()).collect()
    }

    pub fn init(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        shader: wgpu::ShaderModule,
        _config: &OrigamiConfig,
    ) -> Result<Self, KintaroError> {
        let vertices = Self::new_random_vertices(20);
        let num_vertices = vertices.len() as u32;
        let indices = Self::new_random_indices(num_vertices as u32, 30);

        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        });

        let uniforms = crate::origami::uniforms::OrigamiUniforms::new();
        // uniforms.update_view_proj(&camera, &projection);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
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

        let render_pipeline =
            create_origami_render_pipeline(device, &shader, &uniform_bind_group_layout, format);

        Ok(Self {
            vertices,
            vertex_buffer,
            index_buffer,
            num_indices,
            num_vertices,
            uniforms,
            uniform_bind_group,
            uniform_buffer,
            render_pipeline,
        })
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _size: (u32, u32),
        view: &wgpu::TextureView,
        clear: bool,
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // let clear_color = new_clear_color();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                // This is what [[location(0)]] in the fragment shader targets
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        // load: if clear {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.03,
                            g: 0.01,
                            b: 0.04,
                            a: 1.0,
                        }),
                        // } else {
                        // wgpu::LoadOp::Load
                        // },
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..2);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        queue.submit(std::iter::once(encoder.finish()));
    }
}
