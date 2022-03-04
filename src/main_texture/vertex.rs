use super::types::SurfaceVertex;
use wgpu::util::DeviceExt;

pub const SURFACE_VERTICES: &[SurfaceVertex] = &[
    SurfaceVertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    SurfaceVertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    SurfaceVertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    SurfaceVertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
];

pub const SURFACE_INDICES: &[u16] = &[0, 1, 3, 1, 2, 3, /* padding */ 0];

pub fn make_buffers(device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Surface Vertex Buffer"),
        contents: bytemuck::cast_slice(SURFACE_VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Surface Index Buffer"),
        contents: bytemuck::cast_slice(SURFACE_INDICES),
        usage: wgpu::BufferUsages::INDEX,
    });

    (vertex_buffer, index_buffer)
}
