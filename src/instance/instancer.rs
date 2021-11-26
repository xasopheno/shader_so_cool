use super::Instance;
use crate::canvas::Canvas;
use crate::op_stream::Op4D;
use cgmath::Rotation3;
use cgmath::Vector3;
use kintaro_egui_lib::InstanceMul;
use rand::Rng;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SimpleInstancer {}

pub trait Instancer: dyn_clone::DynClone + Debug {
    fn update_instance(&self, instance: &mut Instance, dt: f32);
    fn op4d_to_instance(
        &self,
        instance_mul: &InstanceMul,
        op4d: &Op4D,
        canvas: &Canvas,
    ) -> Instance;
}
dyn_clone::clone_trait_object!(Instancer);

impl Instancer for SimpleInstancer {
    fn update_instance(&self, instance: &mut Instance, dt: f32) {
        instance.life -= dt * 0.1;
        // instance.position.y += f32::sin(3.0 * (2.0 - instance.life));
        instance.position.x += 800.0 * (2.0 - instance.life) * f32::signum(instance.position.x);
        // instance.position.y += 700.0 * (2.0 - instance.life) * f32::signum(instance.position.y);
        // f32::sin(dt * 0.1 * f32::sin(instance.position.x / instance.position.y) * f32::tan(instance.life));
    }

    fn op4d_to_instance(
        &self,
        instance_mul: &InstanceMul,
        op4d: &Op4D,
        canvas: &Canvas,
    ) -> Instance {
        let mut rng = rand::thread_rng();
        let rotation = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::unit_x(),
            // cgmath::Deg(0.0),
            cgmath::Deg(rng.gen_range(-0.3..0.3)),
        );
        let n_row = canvas.n_row;
        let n_column = canvas.n_column;
        let x = -op4d.x as f32 * instance_mul.x;
        let y = op4d.y as f32 * instance_mul.y;
        let z = op4d.z as f32 * instance_mul.z;
        let length = op4d.l as f32 * instance_mul.length;
        let life = 1.0 * instance_mul.life;
        let size = instance_mul.size * f32::max(z, 0.2);
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
            ) - canvas.instance_displacement,
            rotation,
            life,
            size,
            length,
            names: op4d.names.to_owned(),
        }
    }
}
