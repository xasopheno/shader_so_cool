use wgpu::{BindGroup, BindGroupLayout, RenderPipeline};

use super::{image_texture::ImageTexture, image_vertex::ImageVertex};

pub fn create_image_render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    image_texture: &ImageTexture,
) -> (wgpu::BindGroup, wgpu::RenderPipeline) {
    let shader = make_shader(&device);
    let image_bind_group_layout = make_image_bind_group_layout(&device);
    let image_bind_group = make_image_bind_group(&device, &image_bind_group_layout, image_texture);

    let render_pipeline_layout = make_render_pipeline_layout(&device, &image_bind_group_layout);
    let render_pipeline = make_render_pipeline(&device, &render_pipeline_layout, &shader, format);

    (image_bind_group, render_pipeline)
}

fn make_image_bind_group_layout(device: &wgpu::Device) -> BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: true,
                },
                count: None,
            },
        ],
        label: Some("Image Texture Bind Group Layout"),
    })
}

fn make_image_bind_group(
    device: &wgpu::Device,
    image_bind_group_layout: &BindGroupLayout,
    image_texture: &ImageTexture,
) -> BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &image_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&image_texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&image_texture.sampler),
            },
        ],
        label: Some("Image Bind Group"),
    })
}

fn make_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Image Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("image.wgsl").into()),
    })
}

fn make_render_pipeline_layout(
    device: &wgpu::Device,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Image Render Pipeline Layout"),
        bind_group_layouts: &[texture_bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn make_render_pipeline(
    device: &wgpu::Device,
    render_pipeline_layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
) -> RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Image Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[ImageVertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
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
    })
}
