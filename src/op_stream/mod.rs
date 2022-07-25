pub mod renderpasses;
use crate::instance::Instance;
use cgmath::{Rotation3, Vector3};
use kintaro_egui_lib::InstanceMul;
use rand::Rng;
pub use weresocool::core::{
    generation::{EventType, Op4D},
    manager::VisEvent,
};

#[derive(Debug, Clone)]
pub struct OpReceiver {
    pub ops: opmap::OpMap<Op4D>,
    pub cache: opmap::OpMap<Op4D>,
    pub channel: Option<crossbeam_channel::Receiver<VisEvent>>,
}

pub trait GetOps {
    fn init(ops: Option<Vec<Op4D>>, channel: Option<crossbeam_channel::Receiver<VisEvent>>)
        -> Self;
    fn get_batch(&mut self, t: f32, name: &str) -> Vec<Op4D>;
    fn reset(&mut self);
    fn receive(&mut self);
    fn clear_cache(&mut self);
}

impl GetOps for OpReceiver {
    fn init(
        ops: Option<Vec<Op4D>>,
        channel: Option<crossbeam_channel::Receiver<VisEvent>>,
    ) -> Self {
        let mut results = opmap::OpMap::default();

        if let Some(o) = ops {
            o.iter().for_each(|op| {
                let name = if let Some(last) = op.names.last() {
                    last
                } else {
                    "nameless"
                };

                results.insert(name, op.clone());
            });
        }

        Self {
            ops: results,
            cache: opmap::OpMap::default(),
            channel,
        }
    }

    fn reset(&mut self) {
        self.ops = opmap::OpMap::default();
    }

    fn receive(&mut self) {
        if let Some(channel) = &self.channel {
            let new_ops = if let Ok(vis_event) = channel.try_recv() {
                match vis_event {
                    VisEvent::Ops(ops) => ops,
                    VisEvent::Reset => {
                        self.reset();
                        opmap::OpMap::default()
                    }
                }
            } else {
                opmap::OpMap::default()
            };

            self.ops
                .join(new_ops, |a, b| a.t.partial_cmp(&b.t).unwrap());
        }
    }

    fn get_batch(&mut self, t: f32, name: &str) -> Vec<Op4D> {
        if let Some(cached) = self.cache.get(name) {
            cached.to_owned()
        } else {
            let ops = self.ops.drain(name, |v| (v.t as f32) < t);
            if ops.len() > 0 {
                self.cache.set(name, ops.clone());
            };
            ops
        }
    }

    fn clear_cache(&mut self) {
        self.cache = opmap::OpMap::default();
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
        self,
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
        self,
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
            names: self.names,
        }
    }
}
