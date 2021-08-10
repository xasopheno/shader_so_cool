use rand::prelude::*;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub direction: [f32; 3],
    pub velocity: f32,
    // pub decay: f32,
}

pub fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsage::VERTEX,
    })
}

pub fn create_index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsage::INDEX,
    })
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Self {
            position: [x, y, z],
            color: [0.0, 0.0, 0.0],
            direction: [r(), r(), r()],
            velocity: r(),
            // decay: 0.004,
        }
    }
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Self {
            position: [r() * 1.0, r() * 1.0, r() * 1.0],
            color: [r(), r(), r()],
            direction: [r(), r(), r()],
            velocity: r() * 8.0,
            // decay: 0.004,
        }
    }
    pub fn update(&mut self, clear_color: (f64, f64, f64)) {
        // self.position[0] += self.velocity * self.direction[0] * 0.01;
        // self.position[1] += self.velocity * self.direction[1] * 0.01;
        // self.position[2] += self.velocity * self.direction[2] * 0.01;
        // if clear_color.0 < self.color[0] as f64 {
        // self.color[0] = f32::max(clear_color.0 as f32, self.color[0] - self.decay);
        // } else {
        // self.color[0] = f32::min(clear_color.0 as f32, self.color[0] + self.decay);
        // }
        // if clear_color.1 < self.color[1] as f64 {
        // self.color[1] = f32::max(clear_color.1 as f32, self.color[1] - self.decay);
        // } else {
        // self.color[1] = f32::min(clear_color.1 as f32, self.color[1] + self.decay);
        // }
        // if clear_color.2 < self.color[2] as f64 {
        // self.color[2] = f32::max(clear_color.2 as f32, self.color[2] - self.decay);
        // } else {
        // self.color[2] = f32::min(clear_color.2 as f32, self.color[2] + self.decay);
        // }
        // self.color[1] = f32::max(clear_color.1 as f32, self.color[1] - 0.001);
        // self.color[2] = f32::max(clear_color.2 as f32, self.color[2] - 0.001);
        // self.color[3] = f32::max(0.0, self.color[3] - 0.006);
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// unsafe impl bytemuck::Pod for Vertex {}
// unsafe impl bytemuck::Zeroable for Vertex {}
