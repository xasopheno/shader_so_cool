use bytemuck;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ToyUniforms {
    pub width: f32,
    pub height: f32,
    pub frame: f32,
    pub time: f32,
}

impl ToyUniforms {
    pub fn new(
        device: &wgpu::Device,
    ) -> (Self, wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniforms = Self {
            width: 10.0,
            height: 10.0,
            frame: 0.0,
            time: 0.0,
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
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
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

    pub fn update_uniforms(&mut self, window: winit::window::Window, start: std::time::Instant) {
        self.width = window.inner_size().width as _;
        self.height = window.inner_size().height as _;
        self.frame += 1.0;
        self.time = start.elapsed().as_secs_f32();
    }
}
