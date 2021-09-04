use crate::{
    camera::{Camera, CameraController, Projection},
    config::Config,
    helpers::make_color_attachments,
    instance::{make_instance_buffer, make_instances_and_instance_buffer, Instance},
    render_op::{OpStream, ToInstance},
    render_pipleline::create_render_pipeline,
    state::{canvas_info, Canvas},
    vertex::{create_index_buffer, create_vertex_buffer, make_vertex_buffer, Vertex},
};

const U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

pub struct Setup {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniforms: crate::uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
}

pub struct PrintState {
    pub config: Config,
    pub size: (u32, u32),
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertices: Vec<Vertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub vertices_fn: fn() -> Vec<Vertex>,
    pub indices_fn: fn(u16) -> Vec<u16>,
    pub uniforms: crate::uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
    pub count: u32,
    pub op_stream: OpStream,
    pub time_elapsed: std::time::Duration,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub projection: Projection,
    pub canvas: Canvas,
    pub clear_color: (f64, f64, f64),
}

fn make_output_buffer(device: &wgpu::Device, size: (u32, u32)) -> wgpu::Buffer {
    let output_buffer_size = (U32_SIZE * size.0 * size.1) as wgpu::BufferAddress;
    let output_buffer_desc = wgpu::BufferDescriptor {
        size: output_buffer_size,
        usage: wgpu::BufferUsage::COPY_DST
        // this tells wpgu that we want to read this buffer from the cpu
        | wgpu::BufferUsage::MAP_READ,
        label: None,
        mapped_at_creation: false,
    };
    device.create_buffer(&output_buffer_desc)
}

async fn setup(texture_width: u32, texture_height: u32, config: &Config) -> Setup {
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
        texture_view,
        render_pipeline,
        uniforms,
        uniform_buffer,
        uniform_bind_group,
    }
}

impl PrintState {
    pub async fn init(config: Config) -> PrintState {
        let texture_width = 1792;
        let texture_height = 1120;
        println!("{}/{}", texture_width, texture_height);
        let Setup {
            device,
            queue,
            texture,
            texture_view,
            render_pipeline,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        } = setup(texture_width, texture_height, &config).await;

        let op_stream = crate::render_op::OpStream::from_json(&config.filename);
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
            config,
            size: (texture_width, texture_height),
            vertices_fn: crate::helpers::new_random_vertices,
            indices_fn: crate::helpers::new_random_indices,
            device,
            queue,
            texture,
            texture_view,
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
            time_elapsed: std::time::Duration::from_millis(0),
            canvas,
            camera,
            camera_controller,
            projection,
            clear_color: crate::helpers::new_random_clear_color(),
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
        let dt2 = std::time::Duration::from_millis(100);

        self.instances.append(&mut new_instances);
        self.instances.iter_mut().for_each(|i| {
            i.update_state(dt2.as_secs_f32());
        });

        self.instances.retain(|i| i.life > 0.0);
        self.instance_buffer =
            make_instance_buffer(&self.instances, (self.size.0, self.size.1), &self.device);
        self.count += 1;
        if self.count % 200 == 0 {
            self.vertices = (self.vertices_fn)();
            // self.clear_color = crate::helpers::new_random_clear_color();
        }
        // self.vertices.par_iter_mut().for_each(|v| v.update());
        self.camera_controller.update_camera(&mut self.camera, dt2);
        self.uniforms
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub async fn render(&mut self) {
        let dt = std::time::Duration::from_millis(100);
        self.time_elapsed += dt;
        // dbg!(self.time_elapsed);
        self.update(self.time_elapsed);
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[make_color_attachments(
                    &self.texture_view,
                    self.config.accumulation,
                )],
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

        let output_buffer = make_output_buffer(&self.device, self.size);
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(U32_SIZE * self.size.0),
                    rows_per_image: std::num::NonZeroU32::new(self.size.1),
                },
            },
            wgpu::Extent3d {
                width: self.size.0,
                height: self.size.1,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit(Some(encoder.finish()));

        self.write_img(output_buffer).await;
    }

    async fn write_img(&self, output_buffer: wgpu::Buffer) {
        let buffer_slice = output_buffer.slice(..);

        // NOTE: We have to create the mapping THEN device.poll() before await
        // the future. Otherwise the application will freeze.
        let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
        self.device.poll(wgpu::Maintain::Wait);
        mapping.await.unwrap();

        let data = buffer_slice.get_mapped_range();

        use image::{ImageBuffer, Rgba};
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(self.size.0, self.size.1, data).unwrap();
        let filename = format!("out/{:07}.png", self.count);
        if self.count % 100 == 0 {
            dbg!(&filename);
        }
        buffer.save(filename).unwrap();
        // self.output_buffer.unmap();
    }
}
