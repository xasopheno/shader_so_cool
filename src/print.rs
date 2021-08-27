use wgpu::ShaderModule;

fn setup_shaders(device: &wgpu::Device) -> (ShaderModule, ShaderModule) {
    let vs_src = include_str!("shader.vert");
    let fs_src = include_str!("shader.frag");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vs_spirv = compiler
        .compile_into_spirv(
            vs_src,
            shaderc::ShaderKind::Vertex,
            "shader.vert",
            "main",
            None,
        )
        .unwrap();
    let fs_spirv = compiler
        .compile_into_spirv(
            fs_src,
            shaderc::ShaderKind::Fragment,
            "shader.frag",
            "main",
            None,
        )
        .unwrap();
    let vs_data = wgpu::util::make_spirv(vs_spirv.as_binary_u8());
    let fs_data = wgpu::util::make_spirv(fs_spirv.as_binary_u8());
    let vs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: vs_data,
        flags: wgpu::ShaderFlags::default(),
    });
    let fs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: fs_data,
        flags: wgpu::ShaderFlags::default(),
    });
    (vs_module, fs_module)
}

pub struct Setup<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    pub texture_desc: wgpu::TextureDescriptor<'a>,
}

async fn setup<'a>(texture_width: u32, texture_height: u32) -> Setup<'a> {
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
    Setup {
        device,
        queue,
        texture,
        texture_desc,
    }
}

pub async fn init_printer() {
    let texture_width = 1024 * 2;
    let texture_height = 768 * 2;
    let Setup {
        device,
        queue,
        texture,
        texture_desc,
    } = setup(texture_width, texture_width).await;
    let u32_size = std::mem::size_of::<u32>() as u32;

    let output_buffer_size = (u32_size * texture_width * texture_height) as wgpu::BufferAddress;
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

    let render_pipeline = make_render_pipeline(&device, &texture_desc);

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &texture_view,
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

        render_pass.set_pipeline(&render_pipeline);
        render_pass.draw(0..3, 0..1);
    }
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(u32_size * texture_width),
                rows_per_image: std::num::NonZeroU32::new(texture_height),
            },
        },
        texture_desc.size,
    );
    queue.submit(Some(encoder.finish()));

    write_img(
        ImageBuffer {
            output_buffer,
            texture_width,
            texture_height,
        },
        &device,
    )
    .await;
}

struct ImageBuffer {
    output_buffer: wgpu::Buffer,
    texture_width: u32,
    texture_height: u32,
}

fn make_render_pipeline(
    device: &wgpu::Device,
    texture_desc: &wgpu::TextureDescriptor,
) -> wgpu::RenderPipeline {
    let shaders = setup_shaders(&device);
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shaders.0,
            entry_point: "main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shaders.1,
            entry_point: "main",
            targets: &[wgpu::ColorTargetState {
                format: texture_desc.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            clamp_depth: false,
            conservative: false,
            polygon_mode: wgpu::PolygonMode::Fill,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    })
}

async fn write_img(img: ImageBuffer, device: &wgpu::Device) {
    let buffer_slice = img.output_buffer.slice(..);

    // NOTE: We have to create the mapping THEN device.poll() before await
    // the future. Otherwise the application will freeze.
    let mapping = buffer_slice.map_async(wgpu::MapMode::Read);
    device.poll(wgpu::Maintain::Wait);
    mapping.await.unwrap();

    let data = buffer_slice.get_mapped_range();

    use image::{ImageBuffer, Rgba};
    let buffer =
        ImageBuffer::<Rgba<u8>, _>::from_raw(img.texture_width, img.texture_height, data).unwrap();
    buffer.save("image.png").unwrap();
    img.output_buffer.unmap();
}
