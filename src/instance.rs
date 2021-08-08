use cgmath::{Angle, InnerSpace, Rotation3, Zero};
use rand::prelude::*;
use rand::Rng;
use rayon::prelude::*;

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

pub fn make_instances(size: winit::dpi::PhysicalSize<u32>) -> Vec<Instance> {
    dbg!(size);
    let ratio = size.width as f32 / size.height as f32;
    dbg!(ratio);
    let n_pixels = 20.0;
    let n_row = (n_pixels * ratio) as u32;
    let n_column = (n_pixels / ratio) as u32;
    dbg!(n_row, n_column);
    let instance_displacement: cgmath::Vector3<f32> =
        cgmath::Vector3::new(n_row as f32 - 1.0, n_column as f32 - 1.0, n_pixels * 2.0);

    (0..n_column)
        .into_par_iter()
        .flat_map(|y| {
            (0..n_row).into_par_iter().map(move |x| {
                let mut rng1 = rand::thread_rng();
                let mut rng2 = rand::thread_rng();
                let mut rr = || rng1.gen::<f32>() * n_row as f32;
                let mut rc = || rng2.gen::<f32>() * n_column as f32;
                let position = cgmath::Vector3 {
                    x: x as f32 * 2.0,
                    y: y as f32 * 2.0,
                    z: 0.0 as f32,
                } - instance_displacement;

                let rotation = cgmath::Quaternion::from_axis_angle(
                    cgmath::Vector3::unit_y(),
                    cgmath::Deg(0.0),
                );

                Instance { position, rotation }
            })
        })
        .collect::<Vec<_>>()
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
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
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
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
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}
