use crate::instance::Instance;
use cgmath::{Rotation3, Vector3};
use rand::Rng;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Op4D {
    pub t: f64,
    pub voice: usize,
    pub event: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub l: f64,
}

impl Op4D {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            t: 1.0,
            voice: 1,
            event: 1,
            x: rng.gen_range(0.0..1.0),
            y: rng.gen_range(0.0..1.0),
            z: rng.gen_range(0.0..1.0),
            l: rng.gen_range(0.2..2.0),
        }
    }

    pub fn vec_random(n: usize) -> Vec<Self> {
        (0..n).into_par_iter().map(|_| Op4D::new_random()).collect()
    }

    pub fn into_instance(
        &self,
        displacement: &cgmath::Vector3<f32>,
        n_column: u32,
        n_row: u32,
    ) -> Instance {
        let rotation =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_x(), cgmath::Deg(0.0));
        Instance {
            position: Vector3::new(
                self.x as f32 * n_row as f32 * 2.0,
                self.y as f32 * n_column as f32 * 2.0,
                1.0,
            ) - displacement,
            rotation,
            life: 1.0,
            size: self.z as f32 * 2.0,
        }
    }
}
