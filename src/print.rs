use crate::{
    camera::{Camera, CameraController, Projection},
    config::Config,
    instance::{make_instance_buffer, make_instances_and_instance_buffer, Instance},
    render_op::{OpStream, ToInstance},
    render_pipleline::create_render_pipeline,
    state::{canvas_info, Canvas},
    vertex::{create_index_buffer, create_vertex_buffer, make_vertex_buffer, Vertex},
};

const U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

pub struct Setup<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_desc: wgpu::TextureDescriptor<'a>,
    pub output_buffer: wgpu::Buffer,
    pub texture_view: wgpu::TextureView,
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniforms: crate::uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}

pub struct PrintState<'a> {
    pub size: (u32, u32),
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_desc: wgpu::TextureDescriptor<'a>,
    pub texture_view: wgpu::TextureView,
    pub output_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub uniforms: crate::uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
    pub count: u32,
    pub op_stream: OpStream,
    pub start_time: std::time::Instant,
    pub last_render_time: std::time::Instant,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub projection: Projection,
    // pub vertices_fn: fn() -> Vec<Vertex>,
    // pub indices_fn: fn(u16) -> Vec<u16>,
    pub canvas: Canvas,
    // pub clear_color: (f64, f64, f64),
}

async fn setup<'a>(texture_width: u32, texture_height: u32, config: &Config) -> Setup<'a> {
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
        })
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&Default::default(), None)
        .await
        .unwrap();

    let texture_desc = wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: texture_width,
            height: texture_height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::COPY_SRC | wgpu::TextureUsage::RENDER_ATTACHMENT,
        label: None,
    };
    let texture = device.create_texture(&texture_desc);
    let output_buffer_size = (U32_SIZE * texture_width * texture_height) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsage::COPY_DST
        // this tells wpgu that we want to read this buffer from the cpu
        | wgpu::BufferUsage::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    let output_buffer = device.create_buffer(&output_buffer_desc);
    let texture_view = texture.create_view(&Default::default());

    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        flags: wgpu::ShaderFlags::all(),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
        crate::uniforms::Uniforms::new(&device);

    let render_pipeline = create_render_pipeline(
        &device,
        &shader,
        &uniform_bind_group_layout,
        texture_desc.format,
    );

    Setup {
        device,
        queue,
        texture,
        texture_desc,
        output_buffer,
        texture_view,
        render_pipeline,
        uniforms,
        uniform_buffer,
        uniform_bind_group,
    }
}

impl<'a> PrintState<'a> {
    pub async fn init(op_stream: OpStream) -> PrintState<'a> {
        let texture_width = 1792 / 2;
        let texture_height = 1120 / 2;
        println!("{}/{}", texture_width, texture_height);
        let config = Config::new();
        let Setup {
            device,
            queue,
            texture,
            texture_desc,
            output_buffer,
            texture_view,
            render_pipeline,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        } = setup(texture_width, texture_height, &config).await;

        let vertices_fn = crate::helpers::new_random_vertices;
        let indices_fn = crate::helpers::new_random_indices;

        let vertices = vertices_fn();
        let num_vertices = vertices.len() as u32;
        let indices = indices_fn(num_vertices as u16);
        let (instances, instance_buffer) =
            make_instances_and_instance_buffer(0, (texture_width, texture_height), &device);

        let (camera, projection, camera_controller) =
            crate::camera::Camera::new((texture_width, texture_height), &config);

        let vertex_buffer = create_vertex_buffer(&device, &vertices.as_slice());
        let index_buffer = create_index_buffer(&device, &indices.as_slice());
        let canvas = canvas_info((texture_width, texture_height));

        PrintState {
            size: (texture_width, texture_height),
            device,
            queue,
            texture,
            texture_desc,
            texture_view,
            output_buffer,
            render_pipeline,
            vertex_buffer,
            vertices,
            num_vertices,
            index_buffer,
            num_indices: indices.len() as u32,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            instances,
            instance_buffer,
            count: 0,
            op_stream,
            last_render_time: std::time::Instant::now(),
            start_time: std::time::Instant::now(),
            canvas,
            camera,
            camera_controller,
            projection,
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.vertex_buffer = make_vertex_buffer(&self.device, self.vertices.as_slice());
        let mut new_instances: Vec<Instance> = self
            .op_stream
            .get_batch(dt)
            .into_iter()
            .map(|op| {
                op.into_instance(
                    &self.canvas.instance_displacement,
                    self.canvas.n_column,
                    self.canvas.n_row,
                )
            })
            .collect();

        self.instances.append(&mut new_instances);
        self.instances.iter_mut().for_each(|i| {
            i.update_state(dt.as_secs_f32() as f32);
        });

        self.instances.retain(|i| i.life > 0.0);
        self.instance_buffer =
            make_instance_buffer(&self.instances, (self.size.0, self.size.1), &self.device);
        self.count += 1;
        // if self.count % 400 == 0 {
        // self.vertices = (self.vertices_fn)();
        // self.clear_color = crate::helpers::new_random_clear_color();
        // }
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

    pub async fn render(&mut self) {
        let dt = std::time::Duration::from_millis(40);
        self.last_render_time += dt;
        dbg!(self.last_render_time);
        self.update(dt);
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &self.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.02,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            };

            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(U32_SIZE * self.size.0),
                    rows_per_image: std::num::NonZeroU32::new(self.size.1),
                },
            },
            self.texture_desc.size,
        );
        self.queue.submit(Some(encoder.finish()));

        self.write_img().await;
        dbg!("Frame printed");
    }

    async fn write_img(&self) {
        let buffer_slice = self.output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        mapping.await.unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(self.size.0, self.size.1, data).unwrap();
        let filename = format!("out/{:07}.png", self.count);
        dbg!(&filename);
        buffer.save(filename).unwrap();
        self.output_buffer.unmap();
    }
}
