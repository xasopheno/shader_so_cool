use super::PrintState;
use crate::{
    clock::{Clock, PrintClock},
    config::Config,
    instance::make_instances_and_instance_buffer,
    realtime::{canvas_info, RenderPassInput},
    shared::create_render_pipeline,
    vertex::{create_index_buffer, create_vertex_buffer},
};

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

impl PrintState {
    async fn setup(texture_width: u32, texture_height: u32, _config: &Config) -> Setup {
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
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
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
        } = PrintState::setup(texture_width, texture_height, &config).await;

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
            renderpass: RenderPassInput {
                vertex_buffer,
                render_pipeline,
                uniform_bind_group,
                index_buffer,
                instance_buffer,
                instances,
                uniforms,
                uniform_buffer,
                num_vertices,
                num_indices: indices.len() as u32,
                vertices,
                vertices_fn: crate::helpers::new_random_vertices,
                indices_fn: crate::helpers::new_random_indices,
            },
            clock: PrintClock::init(&config),
            config,
            size: (texture_width, texture_height),
            device,
            queue,
            texture,
            texture_view,
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
}
