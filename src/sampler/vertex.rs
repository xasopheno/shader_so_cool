use super::types::SamplerVertex;
use wgpu::util::DeviceExt;

impl SamplerVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<SamplerVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub const SAMPLER_VERTICES: &[SamplerVertex] = &[
    SamplerVertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    SamplerVertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    SamplerVertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    SamplerVertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
];

pub const SAMPLER_INDICES: &[u16] = &[0, 1, 3, 1, 2, 3, /* padding */ 0];

pub fn make_sampler_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, Vec<u16>) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Surface Vertex Buffer"),
        contents: bytemuck::cast_slice(SAMPLER_VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Surface Index Buffer"),
        contents: bytemuck::cast_slice(SAMPLER_INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer, SAMPLER_INDICES.to_vec())
}
