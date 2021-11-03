use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::default::default_cameras;
use crate::color::helpers::*;
use crate::save::ConfigState;
use crate::vertex::shape::{RandIndex, RandPosition, Shape};

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
}

impl Config {
    pub fn new() -> Self {
        let saved = ConfigState::load_saved();
        let cameras = default_cameras(
            if let Ok(ref s) = saved {
                vec![s.camera]
            } else {
                vec![]
            },
            Some((0.0, 20.0, 0.0)),
        );
        let colorsets = colorsets_from_vec_hex_strings(vec![
            // vec!["#6655aa", "#222222"],
            // vec!["#eeaC88", "#121312", "#333333"],
            // vec![
            // "#213CFB", "#310CFA", "#6688aa", "#111111", "#121212", "#101010",
            // ],
            // vec!["#473859", "#222222"],
            // vec!["#300300", "#333333"],
            // vec!["#001931", "#000000", "#222200"],
            // vec!["#660000", "#100101", "#300002"],
            // vec!["#a000a0", "#000000", "#2303aa", "#333333"],
            // vec!["#473859", "#222222"],
            // vec!["#348348", "#112312"],
            // vec!["#0000ee", "#0e000e"],
            vec!["#333333", "#111111", "#777777"],
            vec!["#660000", "#100101", "#300002", "#100001", "#010210"],
            vec!["#473850", "#222222", "#001001"],
            vec!["#112112", "#000033"],
            vec!["#ff00ff", "#000000"],
            vec!["#38881a", "#333333"],
            vec!["#aa10e4", "#333333"],
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
            instance_mul: if let Ok(s) = saved {
                s.instance_mul
            } else {
                InstanceMul {
                    x: 9.0,
                    y: 19.0,
                    z: 1.0,
                    life: 2.0,
                    size: 23.0,
                    length: 1.0,
                }
            },
            cameras,
        }
    }
}
