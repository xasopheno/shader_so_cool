pub mod renderpasses;
use crate::{application::Visual, instance::Instance};
use cgmath::{Rotation3, Vector3};
use kintaro_egui_lib::InstanceMul;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
pub use weresocool::generation::json::{EventType, Op4D};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpStream {
    pub ops: Vec<Op4D>,
    pub length: f32,
    pub names: Vec<String>,
}

impl OpStream {
    pub fn from_json(filename: &str) -> Vec<OpStream> {
        let data = std::fs::read_to_string(format!("./{}.socool.json", filename))
            .expect("Unable to read file");

        let deserialized: OpStream = serde_json::from_str(&data).unwrap();
        let mut op_streams = BTreeMap::<Vec<String>, Vec<Op4D>>::new();
        deserialized.ops.iter().for_each(|op| {
            if op.names.is_empty() {
                let stream = op_streams.entry(vec!["nameless".into()]).or_insert(vec![]);
                stream.push(op.clone());
            } else {
                let names = &op.names;
                let stream = op_streams.entry(names.to_owned()).or_insert(vec![]);
                stream.push(op.clone());
            }
        });

        op_streams
            .into_iter()
            .map(|(names, ops)| OpStream {
                ops,
                length: deserialized.length,
                names,
            })
            .collect()
    }

    pub fn from_vec_op4d(v: &Visual) -> Vec<OpStream> {
        let mut op_streams = BTreeMap::<Vec<String>, Vec<Op4D>>::new();
        v.visual.iter().for_each(|op| {
            if op.names.is_empty() {
                let stream = op_streams.entry(vec!["nameless".into()]).or_insert(vec![]);
                stream.push(op.clone());
            } else {
                let names = &op.names;
                let stream = op_streams.entry(names.to_owned()).or_insert(vec![]);
                stream.push(op.clone());
            }
        });

        op_streams
            .into_iter()
            .map(|(names, ops)| OpStream {
                ops,
                length: v.length,
                names,
            })
            .collect()
    }
    pub fn get_batch(&mut self, t: f32) -> Vec<Op4D> {
        let result: Vec<Op4D> = self
            .ops
            .iter()
            .take_while(|op| op.t < t.into())
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
        mul: InstanceMul,
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
            names: vec![],
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
        mul: InstanceMul,
    ) -> Instance {
        let mut rng = rand::thread_rng();
        let rotation = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::unit_x(),
            // cgmath::Deg(0.0),
            cgmath::Deg(rng.gen_range(-0.3..0.3)),
        );
        let x = -self.x as f32 * mul.x;
        let y = self.y as f32 * mul.y;
        let z = self.z as f32 * mul.z;
        let length = self.l as f32 * mul.length;
        let life = 1.0 * mul.life;
        let size = mul.size * f32::max(z, 0.2);
        Instance {
            position: Vector3::new(
                // n_row as f32 * 1.0 / 2.0 * f32::powi(x, 2),
                n_row as f32 * x,
                n_column as f32 * y,
                // n_row as f32 * x / 3.0 * y,
                // sin(x ^ 1.0 / 2.0 - y ^ -2.0),
                // n_row as f32 * 1.0 * y / x,
                // n_row as f32 * (self.x * self.x) as f32 * 2.0 / self.y as f32,
                // n_column as f32 * (self.y * self.y) as f32 / 2.0 * 10.0,
                // (self.x as f32 * n_row as f32 * 4.0 / self.y as f32) / 3.0,
                // * f32::sin(1000.0 * self.y as f32),
                // self.y as f32 * n_column as f32 * 9.0,
                1.0,
            ) - displacement,
            rotation,
            life,
            size,
            length,
            names: self.names.to_owned(),
        }
    }
}
