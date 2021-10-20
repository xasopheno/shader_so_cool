use kintaro_egui_lib::InstanceMul;

use crate::camera::default::default_cameras;
use crate::color::helpers::*;
use crate::vertex::shape::{RandIndex, RandPosition, Shape};

#[derive(Clone, Copy)]
pub struct CameraConfig {
    pub position: (f32, f32, f32),
    pub yaw: f32,
    pub pitch: f32,
    pub index: usize,
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
}

impl Config {
    pub fn new() -> Self {
        let colorsets = colorsets_from_vec_hex_strings(vec![
            vec!["#6655aa", "#222222"],
            vec!["#eeaC88", "#121312", "#333333"],
            vec![
                "#213CFB", "#310CFA", "#6688aa", "#111111", "#121212", "#101010",
            ],
            vec!["#473859", "#222222"],
            vec!["#300300", "#333333"],
            vec!["#001931", "#000000", "#222200"],
        ]);
        Config {
            accumulation: false,
            filename: "kintaro".into(),
            volume: 0.20,
            window_size: (1792 * 4, 1120 * 4),
            shape: Shape {
                n_vertices: 70,
                n_indices: 70,
                position: Box::new(RandPosition),
                color: Box::new(colorsets),
                indices: Box::new(RandIndex),
            },
            instance_mul: InstanceMul {
                x: 9.0,
                y: 17.0,
                z: 1.0,
                life: 2.0,
                size: 23.0,
                length: 1.0,
            },
            cameras: default_cameras(Some((0.0, 20.0, 0.0))),
        }
    }
}
