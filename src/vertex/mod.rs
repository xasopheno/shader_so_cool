use rand::prelude::*;
use rand::seq::SliceRandom;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub direction: [f32; 3],
    pub velocity: f32,
}

pub type ColorSet = &'static Vec<Color>;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub shade: f32,
}

pub fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

pub fn create_index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsages::INDEX,
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
        }
    }
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Self {
            position: [r() * 1.0, r() * 1.0, r() * 3.0],
            color: [r() * 0.7, r() * 0.7, 1.0],
            direction: [r(), r(), r()],
            velocity: r() * 0.4,
        }
    }

    pub fn new_random_from_colorset(colorset: ColorSet) -> Self {
        let color = colorset.choose(&mut rand::thread_rng()).unwrap();
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        let shade = color.shade;

        Self {
            position: [r() * 1.0, r() * 1.0, r() * 3.0],
            color: [color.r * shade, color.g * shade, color.b * shade],
            direction: [r(), r(), r()],
            velocity: r() * 0.4,
        }
    }

    #[allow(dead_code)]
    pub fn update(&mut self) {
        self.position[0] += self.velocity * self.direction[0] * 0.02;
        self.position[1] += self.velocity * self.direction[1] * 0.02;
        self.position[2] += self.velocity * self.direction[2] * 0.02;
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
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

pub fn make_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsages::VERTEX,
    })
}

// unsafe impl bytemuck::Pod for Vertex {}
// unsafe impl bytemuck::Zeroable for Vertex {}
