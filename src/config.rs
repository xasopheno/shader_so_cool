use crate::color::{helpers::*, Color, ColorSet, RandColor};
use crate::shared::helpers::{new_random_indices, new_random_vertices};
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
}

impl Config {
    pub fn new() -> Self {
        let colorsets = colorsets_from_vec_hex_strings(vec![
            vec!["#4778B8", "#333333"],
            vec!["#FA0C8F", "#121312", "#333333"],
            vec!["#325380", "#333333"],
            vec!["#473859", "#222222"],
            vec!["#ababab", "#291931"],
            vec![
                "#213CFB", "#310CFA", "#FADE19", "#111111", "#121212", "#101010",
            ],
        ]);
        let offset = (0.0, -20.0, 0.0);
        Config {
            accumulation: false,
            filename: "kintaro".into(),
            volume: 0.20,
            window_size: (1792, 1120),
            shape: Shape {
                n_vertices: 30,
                n_indices: 30,
                position: Box::new(RandPosition),
                color: Box::new(colorsets),
                indices: Box::new(RandIndex),
            },
            cameras: vec![
                CameraConfig {
                    index: 0,
                    position: (0.0 + offset.0, 90.0 + offset.1, 200.0 + offset.2),
                    yaw: -90.0,
                    pitch: 0.0,
                },
                CameraConfig {
                    index: 1,
                    position: (310.0 + offset.0, 83.0 + offset.1, 77.0 + offset.2),
                    yaw: -142.0,
                    pitch: 1.77,
                },
                CameraConfig {
                    index: 2,
                    position: (-218.0 + offset.0, -40.0 + offset.1, -89.0 + offset.2),
                    yaw: -4.0,
                    pitch: 31.8,
                },
                CameraConfig {
                    index: 3,
                    position: (-116.2 + offset.0, 36.0 + offset.1, 106.0 + offset.2),
                    yaw: -56.11,
                    pitch: 5.917,
                },
                CameraConfig {
                    index: 4,
                    position: (0.0 + offset.0, 80.0 + offset.1, 400.0 + offset.2),
                    yaw: -90.0,
                    pitch: 11.0,
                },
                CameraConfig {
                    index: 5,
                    position: (0.0 + offset.0, 670.0 + offset.1, -226.0 + offset.2),
                    yaw: 0.0,
                    pitch: -90.0,
                },
            ],
        }
    }
}
