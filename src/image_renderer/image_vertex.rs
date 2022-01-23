use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ImageVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl ImageVertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ImageVertex>() as wgpu::BufferAddress,
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

pub fn make_image_vertices_and_indices(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
    let image_vertices: &[ImageVertex] = &[
        ImageVertex {
            position: [1.0, 1.0, 0.0],
            tex_coords: [1.0, 0.0],
        }, // A
        ImageVertex {
            position: [-1.0, 1.0, 0.0],
            tex_coords: [0.0, 0.0],
        }, // B
        ImageVertex {
            position: [-1.0, -1.0, 0.0],
            tex_coords: [0.0, 1.0],
        }, // C
        ImageVertex {
            position: [1.0, -1.0, 0.0],
            tex_coords: [1.0, 1.0],
        }, // D
    ];

    let image_indices: &[u16] = &[0, 1, 3, 1, 2, 3, /* padding */ 0];
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Image Vertex Buffer"),
        contents: bytemuck::cast_slice(image_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Image Index Buffer"),
        contents: bytemuck::cast_slice(image_indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    let num_indices = image_indices.len() as u32;

    (vertex_buffer, index_buffer, num_indices)
}
