use cgmath::Rotation3;
use rand::Rng;
use rayon::prelude::*;
use wgpu::util::DeviceExt;

use crate::config::Config;

#[derive(Copy, Clone, Debug)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub life: f32,
    pub size: f32,
    pub length: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
    life: f32,
    size: f32,
    length: f32,
}

pub fn make_instances(n: usize, size: (u32, u32)) -> Vec<Instance> {
    let ratio = size.0 as f32 / size.1 as f32;
    let grid_size = 10.0;
    let n_row = (grid_size * ratio) as u32;
    let n_column = grid_size as u32;
    let instance_displacement: cgmath::Vector3<f32> =
        cgmath::Vector3::new(n_row as f32 - 1.0, (n_column - 1) as f32, grid_size * 2.7);

    (0..n)
        .into_iter()
        .map(move |_x| {
            let mut rng1 = rand::thread_rng();
            let mut rng2 = rand::thread_rng();
            let mut rng3 = rand::thread_rng();
            let mut rr = || rng1.gen::<f32>() * n_row as f32 * 2.0;
            let mut rc = || rng2.gen::<f32>() * n_column as f32 * 2.0;
            let position = cgmath::Vector3 {
                x: rr(),
                y: rc(),
                z: 0.0 as f32,
            } - instance_displacement;

            let rotation =
                cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_x(), cgmath::Deg(0.0));
            let size = rng3.gen::<f32>() * 2.0;

            Instance {
                position,
                rotation,
                life: 1.0,
                size,
                length: 1.0,
            }
        })
        .collect::<Vec<_>>()
}

pub fn make_instance_buffer(
    instances: &Vec<Instance>,
    _size: (u32, u32),
    device: &wgpu::Device,
) -> wgpu::Buffer {
    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsage::VERTEX,
    });

    instance_buffer
}

pub fn make_instances_and_instance_buffer(
    n: usize,
    size: (u32, u32),
    device: &wgpu::Device,
) -> (Vec<Instance>, wgpu::Buffer) {
    let instances = make_instances(n, size);
    let instance_buffer = make_instance_buffer(&instances, size, device);
    (instances, instance_buffer)
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
            life: self.life,
            size: self.size,
            length: self.length,
        }
    }

    pub fn update_state(&mut self, dt: f32) {
        self.life -= dt * 0.001;
    }
}

impl InstanceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in
                // the shader.
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 17]>() as wgpu::BufferAddress,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}
