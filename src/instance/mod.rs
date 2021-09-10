pub mod raw;
use cgmath::Rotation3;
use rand::Rng;
use wgpu::util::DeviceExt;

use self::raw::InstanceRaw;

#[derive(Clone, Debug)]
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub life: f32,
    pub size: f32,
    pub length: f32,
    pub names: Vec<String>,
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
                names: vec![],
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
        self.life -= dt * 0.1;
        self.position.y += f32::sin(3.0 * (2.0 - self.life));
        // f32::sin(dt * 0.1 * f32::sin(self.position.x / self.position.y) * f32::tan(self.life));
    }
}
