use bytemuck;
use wgpu::util::DeviceExt;

use crate::renderable::RGBA;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ToyUniforms {
    pub width: f32,
    pub height: f32,
    pub frame: f32,
    pub time: f32,
    // pub r: f32,
    // pub g: f32,
    // pub b: f32,
    // pub a: f32,
}

impl ToyUniforms {
    pub fn new(
        device: &wgpu::Device,
        // rgba: RGBA,
    ) -> (Self, wgpu::Buffer, wgpu::BindGroupLayout, wgpu::BindGroup) {
        let uniforms = Self {
            width: 1000.0,
            height: 1000.0,
            frame: 0.0,
            time: 0.0,
            // r: rgba.r,
            // g: rgba.g,
            // b: rgba.b,
            // a: rgba.a,
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
            layout: &uniform_bind_group_layout,
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

    pub fn update_uniforms(&mut self, size: (u32, u32), total_elapsed: f32) {
        self.width = size.0 as f32;
        self.height = size.1 as f32;
        self.frame += 1.0;
        self.time = total_elapsed;
    }
}
