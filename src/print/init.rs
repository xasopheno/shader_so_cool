use super::PrintState;
use crate::{
    canvas::Canvas,
    clock::{Clock, PrintClock},
    config::Config,
    instance::make_instances_and_instance_buffer,
    shared::{create_render_pipeline, new_random_clear_color, RenderPassInput},
    vertex::{create_index_buffer, create_vertex_buffer},
};

impl PrintState {
    pub async fn init(config: Config) -> PrintState {
        let texture_width = 1792 * 4;
        let texture_height = 1120 * 4;

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
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
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        println!("{}/{}", texture_width, texture_height);

        let op_streams = crate::render_op::OpStream::from_json(&config.filename);

        let toy = crate::toy::setup_toy(
            &device,
            std::time::Instant::now(),
            (texture_width, texture_height),
            texture_desc.format,
        );

        let renderpasses = op_streams
            .iter()
            .map(|op_stream| {
                let vertices = (config.vertices_fn)();
                let num_vertices = vertices.len() as u32;
                let indices = (config.indices_fn)(num_vertices as u16);
                let (instances, instance_buffer) =
                    make_instances_and_instance_buffer(0, (texture_width, texture_height), &device);
                let (uniforms, uniform_buffer, uniform_bind_group_layout, uniform_bind_group) =
                    crate::uniforms::RealtimeUniforms::new(&device);
                let render_pipeline = create_render_pipeline(
                    &device,
                    &shader,
                    &uniform_bind_group_layout,
                    texture_desc.format,
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
        PrintState {
            renderpasses,
            toy: Some(toy),
            clock: PrintClock::init(&config),

            canvas: Canvas::init((texture_width, texture_height)),
            camera: crate::camera::Camera::new(
                &config.cameras[4],
                (texture_width, texture_height),
                &config,
            ),
            config,
            size: (texture_width, texture_height),
            device,
            queue,
            texture,
            texture_view,
            count: 0,
            time_elapsed: std::time::Duration::from_millis(0),
            clear_color: new_random_clear_color(),
        }
    }
}
