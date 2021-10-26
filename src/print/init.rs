use super::PrintState;
use crate::{
    canvas::Canvas,
    clock::{Clock, PrintClock},
    config::Config,
    instance::make_instances_and_instance_buffer,
    shared::{create_render_pipeline, new_random_clear_color, RenderPassInput},
    vertex::{create_index_buffer, create_vertex_buffer, shape::ShapeGenResult},
};

impl PrintState {
    pub async fn init(config: &mut Config) -> PrintState {
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
                width: config.window_size.0,
                height: config.window_size.1,
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

        println!("{}/{}", config.window_size.0, config.window_size.1);

        let op_streams = crate::render_op::OpStream::from_json(&config.filename);

        let toy = crate::toy::setup_toy(
            &device,
            std::time::Instant::now(),
            config.window_size,
            texture_desc.format,
        );

        let renderpasses = op_streams
            .iter()
            .map(|op_stream| {
                let ShapeGenResult { vertices, indices } = config.shape.gen();
                config.shape.update();
                let (instances, instance_buffer) =
                    make_instances_and_instance_buffer(0, config.window_size, &device);
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
                    uniform_buffer,
                    uniforms,
                    shape: config.shape.clone(),
                    render_pipeline,
                }
            })
            .collect();
        PrintState {
            renderpasses,
            toy: Some(toy),
            clock: PrintClock::init(&config),

            canvas: Canvas::init(config.window_size),
            camera: crate::camera::Camera::new(&config.cameras[0], config.window_size, &config, 0),
            size: config.window_size,
            config: config.clone(),
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
