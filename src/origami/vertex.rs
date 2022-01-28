use rand::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OrigamiVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub direction: [f32; 3],
    pub velocity: f32,
}

impl OrigamiVertex {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let mut r = || rng.gen::<f32>() * 2.0 - 1.0;
        Self {
            position: [r(), r(), r()],
            color: [r(), r(), r()],
            direction: [r(), r(), r()],
            velocity: r(),
        }
    }
    pub fn update(&mut self) {
        // self.position[0] += self.velocity * self.direction[0] * 0.001;
        // self.position[1] += self.velocity * self.direction[0] * 0.001;
        // self.position[2] += self.velocity * self.direction[0] * 0.001;
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<OrigamiVertex>() as wgpu::BufferAddress,
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

// unsafe impl bytemuck::Pod for Vertex {}
// unsafe impl bytemuck::Zeroable for Vertex {}
