use super::{
    instance::SamplerInstanceRaw,
    types::{Sampler, SamplerVertex},
};
use crate::{frame::vertex::make_square_buffers, shader::make_shader};
use anyhow::Result;

impl Sampler {
    pub fn new(
        device: &wgpu::Device,
        size: (u32, u32),
        format: wgpu::TextureFormat,
    ) -> Result<Self> {
        let main_shader = make_shader(&device, "./src/sampler/sampler_shader.wgsl");

        let texture =
            crate::frame::texture::Texture::new(&device, (size.0, size.1), "sampler_frame", format)
                .unwrap();

        let texture_bind_group_layout = make_texture_bind_group_layout(&device);

        let texture_bind_group = crate::frame::setup::make_frame_texture_bind_group(
            device,
            &texture_bind_group_layout,
            &texture,
        );

        let render_pipeline = make_render_pipeline(
            &device,
            &texture_bind_group_layout,
            &main_shader?,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        );

        let (vertex_buffer, index_buffer, indices) = make_square_buffers(device);

        let instances = super::instance::make_instances(device);

        Ok(Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            texture,
            texture_bind_group_layout,
            texture_bind_group,
            indices,
            instances,
        })
    }
}

pub fn make_frame_texture_bind_group(
    device: &wgpu::Device,
    frame_bind_group_layout: &wgpu::BindGroupLayout,
    frame: &crate::frame::texture::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: frame_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&frame.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&frame.sampler),
            },
        ],
        label: Some("main_bind_group"),
    })
}

pub fn make_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("frame_bind_group_layout"),
    })
}

pub fn make_render_pipeline(
    device: &wgpu::Device,
    frame_bind_group_layout: &wgpu::BindGroupLayout,
    shader: &wgpu::ShaderModule,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&frame_bind_group_layout],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[SamplerVertex::desc(), SamplerInstanceRaw::desc()],
            // buffers: &[SamplerVertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
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
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    })
}
