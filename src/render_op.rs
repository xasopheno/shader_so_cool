use crate::instance::Instance;
use cgmath::{Rotation3, Vector3};
use rand::Rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
pub use weresocool::generation::json::{EventType, Op4D};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpStream {
    pub ops: Vec<Op4D>,
    pub length: f32,
}

impl OpStream {
    pub fn from_json(filename: &str) -> OpStream {
        let data = std::fs::read_to_string(format!("./{}.socool.json", filename))
            .expect("Unable to read file");

        let deserialized: OpStream = serde_json::from_str(&data).unwrap();
        deserialized
    }
    pub fn get_batch(&mut self, t: std::time::Duration) -> Vec<Op4D> {
        let result: Vec<Op4D> = self
            .ops
            .iter()
            .take_while(|op| op.t < t.as_secs_f64())
            .map(|x| x.to_owned())
            .collect();
        for _ in 0..result.len() {
            self.ops.remove(0);
        }

        result
    }
}

pub trait ToInstance {
    fn new_random(t: f64) -> Self;
    fn vec_random(n: usize) -> Vec<Self>
    where
        Self: Sized;
    fn vec_random_ops(t: f64, n: usize) -> Vec<Self>
    where
        Self: Sized;
    fn into_instance(
        &self,
        displacement: &cgmath::Vector3<f32>,
        n_column: u32,
        n_row: u32,
    ) -> Instance;
}

impl ToInstance for Op4D {
    fn new_random(t: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            t,
            voice: 1,
            event: 1,
            event_type: EventType::On,
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
            l: rng.gen_range(0.2..2.0),
        }
    }

    fn vec_random(n: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut rng2 = rand::thread_rng();
        let mut next_op_t = || rng.gen_range(0.0..1.0);
        let mut num_ops = || rng2.gen_range(1..20);
        let mut count = 0.0;
        (0..n)
            .into_iter()
            .flat_map(|_| {
                count += next_op_t();
                Op4D::vec_random_ops(count, num_ops())
            })
            .collect()
    }

    fn vec_random_ops(t: f64, n: usize) -> Vec<Op4D> {
        (0..n).into_iter().map(|_| Op4D::new_random(t)).collect()
    }

    fn into_instance(
        &self,
        displacement: &cgmath::Vector3<f32>,
        n_column: u32,
        n_row: u32,
    ) -> Instance {
        // let mut rng = rand::thread_rng();
        let rotation = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::unit_x(),
            cgmath::Deg(0.0),
            // cgmath::Deg(rng.gen_range(-0.1..0.1)),
        );
        Instance {
            position: Vector3::new(
                self.x as f32 * n_row as f32,
                self.y as f32 * n_column as f32 * 2.0,
                1.0,
            ) - displacement,
            rotation,
            life: 2.0,
            size: 2.0 * self.z as f32,
            length: self.l as f32,
        }
    }
}