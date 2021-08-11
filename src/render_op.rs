use crate::instance::Instance;
use cgmath::{Rotation3, Vector3};

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

impl From<Op4D> for Instance {
    fn from(op: Op4D) -> Self {
        let rotation =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_x(), cgmath::Deg(0.0));
        Instance {
            position: Vector3::new(op.x as f32, op.y as f32, 1.0 as f32),
            rotation,
            life: 1.0,
            size: op.z as f32,
        }
    }
}
