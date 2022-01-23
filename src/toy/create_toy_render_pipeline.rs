pub fn create_toy_render_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: "Toy Pipeline Layout".into(),
        bind_group_layouts: &[uniform_bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        multiview: None,
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: "fs_main",
            // targets: &[format.into()],
            targets: &[wgpu::ColorTargetState {
                format,
                blend: Some(
                    wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING, // {
                                                                    // color: wgpu::BlendComponent::OVER,
                                                                    // alpha: wgpu::BlendComponent::OVER,
                                                                    // }
                ),
                // blend: Some(wgpu::BlendState {
                // color: BlendComponent {
                // src_factor: wgpu::BlendFactor::SrcAlpha,
                // dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                // operation: wgpu::BlendOperation::Add,
                // },
                // alpha: BlendComponent {
                // src_factor: wgpu::BlendFactor::One,
                // dst_factor: wgpu::BlendFactor::One,
                // operation: wgpu::BlendOperation::Add,
                // },
                // }),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),

        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
    })
}
