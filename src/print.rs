use crate::{
    config::Config,
    instance::{make_instances_and_instance_buffer, Instance},
    render_pipleline::create_render_pipeline,
    vertex::{create_index_buffer, create_vertex_buffer, Vertex},
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
    //
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

pub struct PrintState<'a> {
    pub size: (u32, u32),
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_desc: wgpu::TextureDescriptor<'a>,
    pub texture_view: wgpu::TextureView,
    pub output_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub last_render_time: std::time::Instant,
    //
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
    // pub projection: Projection,
    // pub camera_controller: CameraController,
    // pub last_render_time: std::time::Instant,
    // pub start_time: std::time::Instant,
    // pub vertices_fn: fn() -> Vec<Vertex>,
    // pub indices_fn: fn(u16) -> Vec<u16>,
    // pub canvas: Canvas,
    // pub clear_color: (f64, f64, f64),
    // pub count: u32,
    // pub camera: Camera,
}

impl<'a> PrintState<'a> {
    pub async fn init() -> PrintState<'a> {
        let texture_width = 1024 * 2;
        let texture_height = 768 * 2;
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

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            flags: wgpu::ShaderFlags::all(),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

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
        // let canvas = canvas_info(window.inner_size());

        PrintState {
            size: (texture_width, texture_height),
            device,
            queue,
            texture,
            texture_desc,
            texture_view,
            output_buffer,
            render_pipeline,
            last_render_time: std::time::Instant::now(),
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
        }
    }

    pub async fn render(mut self) {
        let now = std::time::Instant::now();
        let dt = now - self.last_render_time;
        self.last_render_time = now;
        // dbg!(self.last_render_time);
        // self.update(dt);
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
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
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

        write_img(
            &ImageBuffer {
                output_buffer: self.output_buffer,
                size: self.size,
            },
            &self.device,
        )
        .await;
        dbg!("Frame printed");
    }
}

struct ImageBuffer {
    output_buffer: wgpu::Buffer,
    size: (u32, u32),
}

async fn write_img(img: &ImageBuffer, device: &wgpu::Device) {
    let buffer_slice = img.output_buffer.slice(..);

    // NOTE: We have to create the mapping THEN device.poll() before await
    // the future. Otherwise the application will freeze.
    let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
    device.poll(wgpu::Maintain::Wait);
    mapping.await.unwrap();

    let data = buffer_slice.get_mapped_range();

    use image::{ImageBuffer, Rgba};
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(img.size.0, img.size.1, data).unwrap();
    buffer.save("image.png").unwrap();
    img.output_buffer.unmap();
}
