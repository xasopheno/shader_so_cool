use bytemuck;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RealtimeUniforms {
    view_proj: [[f32; 4]; 4],
    view_position: [f32; 4],
}
impl RealtimeUniforms {
    pub fn new(
        device: &wgpu::Device,
    ) -> (Self, wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        use cgmath::SquareMatrix;
        let uniforms = Self {
            view_position: [0.0; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &&uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        (
            uniforms,
            uniform_buffer,
            uniform_bind_group_layout,
            uniform_bind_group,
        )
    }

    pub fn update_view_proj(&mut self, view_position: [f32; 4], view_proj: [[f32; 4]; 4]) {
        self.view_position = view_position;
        self.view_proj = view_proj;
    }
}
