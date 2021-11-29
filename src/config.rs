use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::default::default_cameras;
use crate::instance::instancer::{Instancer, SimpleInstancer};
use crate::save::ConfigState;
use crate::vertex::shape::{RandIndex, RandPosition, Shape};
use crate::ColorSets;

impl Default for Config {
    fn default() -> Self {
        let instance_mul = InstanceMul {
            x: 9.0,
            y: 19.0,
            z: 1.0,
            life: 2.0,
            size: 23.0,
            length: 1.0,
        };
        let (cameras, instance_mul) = Config::handle_save(instance_mul);
        Config {
            instance_shader: "./src/shader.wgsl".into(),
            toy_shader: "./src/toy.wgsl".into(),
            instancer: Box::new(SimpleInstancer {}),
            instance_mul,
            accumulation: false,
            filename: "kintaro".into(),
            volume: 0.20,
            window_size: (1792 * 2, 1120 * 2),
            cameras,
            shape: Shape {
                n_vertices: 70,
                n_indices: 70,
                position: Box::new(RandPosition),
                color: Box::new(ColorSets::default()),
                indices: Box::new(RandIndex),
            },
        }
    }
}

impl Config {
    fn handle_save(instance_mul: InstanceMul) -> (Vec<CameraConfig>, InstanceMul) {
        let saved = ConfigState::load_saved();
        let cameras = default_cameras(
            if let Ok(ref s) = saved {
                vec![s.camera]
            } else {
                vec![]
            },
            Some((0.0, 20.0, 0.0)),
        );

        let instance_mul = if let Ok(s) = saved {
            s.instance_mul
        } else {
            instance_mul
        };
        (cameras, instance_mul)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub position: (f32, f32, f32),
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone)]
pub struct Config {
    pub filename: String,
    pub volume: f32,
    pub window_size: (u32, u32),
    pub cameras: Vec<CameraConfig>,
    pub accumulation: bool,
    pub shape: Shape,
    pub instance_mul: InstanceMul,
    pub instancer: Box<dyn Instancer>,
    pub instance_shader: String,
    pub toy_shader: String,
}
