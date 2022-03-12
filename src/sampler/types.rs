use crate::frame::texture::Texture;

pub struct Sampler {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub instances: wgpu::Buffer,
    pub texture: Texture,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub texture_bind_group: wgpu::BindGroup,
    pub indices: Vec<u16>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SamplerVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
