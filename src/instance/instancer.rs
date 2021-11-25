use super::Instance;
use crate::op_stream::Op4D;
use cgmath::Rotation3;
use cgmath::Vector3;
use kintaro_egui_lib::InstanceMul;
use rand::Rng;

pub struct SimpleInstancer {
    instance_mul: InstanceMul,
}

pub trait Instancer {
    fn update_instance(instance: &mut Instance, dt: f32);
    fn op4d_to_instance(
        &mut self,
        op4d: &Op4D,
        displacement: &cgmath::Vector3<f32>,
        n_column: u32,
        n_row: u32,
    ) -> Instance;
}

impl Instancer for SimpleInstancer {
    fn update_instance(instance: &mut Instance, dt: f32) {
        instance.life -= dt * 0.1;
        // instance.position.y += f32::sin(3.0 * (2.0 - instance.life));
        instance.position.x += 800.0 * (2.0 - instance.life) * f32::signum(instance.position.x);
        // instance.position.y += 700.0 * (2.0 - instance.life) * f32::signum(instance.position.y);
        // f32::sin(dt * 0.1 * f32::sin(instance.position.x / instance.position.y) * f32::tan(instance.life));
    }

    fn op4d_to_instance(
        &mut self,
        op4d: &Op4D,
        displacement: &cgmath::Vector3<f32>,
        n_column: u32,
        n_row: u32,
    ) -> Instance {
        let mut rng = rand::thread_rng();
        let rotation = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::unit_x(),
            // cgmath::Deg(0.0),
            cgmath::Deg(rng.gen_range(-0.3..0.3)),
        );
        let x = -op4d.x as f32 * self.instance_mul.x;
        let y = op4d.y as f32 * self.instance_mul.y;
        let z = op4d.z as f32 * self.instance_mul.z;
        let length = op4d.l as f32 * self.instance_mul.length;
        let life = 1.0 * self.instance_mul.life;
        let size = self.instance_mul.size * f32::max(z, 0.2);
        Instance {
            position: Vector3::new(
                // n_row as f32 * 1.0 / 2.0 * f32::powi(x, 2),
                n_row as f32 * x,
                n_column as f32 * y,
                // n_row as f32 * x / 3.0 * y,
                // sin(x ^ 1.0 / 2.0 - y ^ -2.0),
                // n_row as f32 * 1.0 * y / x,
                // n_row as f32 * (op4d.x * op4d.x) as f32 * 2.0 / op4d.y as f32,
                // n_column as f32 * (op4d.y * op4d.y) as f32 / 2.0 * 10.0,
                // (op4d.x as f32 * n_row as f32 * 4.0 / op4d.y as f32) / 3.0,
                // * f32::sin(1000.0 * op4d.y as f32),
                // op4d.y as f32 * n_column as f32 * 9.0,
                1.0,
            ) - displacement,
            rotation,
            life,
            size,
            length,
            names: op4d.names.to_owned(),
        }
    }
}
