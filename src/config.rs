use crate::shared::helpers::{new_random_indices, new_random_vertices};
use crate::vertex::Vertex;

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
    pub vertices_fn: fn() -> Vec<Vertex>,
    pub indices_fn: fn(u16) -> Vec<u16>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            accumulation: false,
            filename: "kintaro".into(),
            volume: 0.05,
            window_size: (1792, 1120),
            vertices_fn: new_random_vertices,
            indices_fn: new_random_indices,
            cameras: vec![
                CameraConfig {
                    index: 0,
                    position: (-27.0, -42.0, 152.0),
                    yaw: -82.0,
                    pitch: 25.0,
                },
                CameraConfig {
                    index: 1,
                    position: (310.0, 83.0, 77.0),
                    yaw: -142.0,
                    pitch: 1.77,
                },
                CameraConfig {
                    index: 2,
                    position: (0.0, 50.0, 200.0),
                    yaw: -90.0,
                    pitch: 0.0,
                },
                CameraConfig {
                    index: 3,
                    position: (-1.0, 0.79, 435.0),
                    yaw: -90.0,
                    pitch: 22.0,
                },
                // CameraConfig {
                    // index: 0,
                    // position: (-22.83299, -5.1967072, -35.540905),
                    // yaw: 16.77102,
                    // pitch: 5.3154497,
                // },
            ]
            //
            // camera: CameraConfig {
            // position: (-116.2, 36.0, 106.0),
            // yaw: -56.11,
            // pitch: 5.917,
            // },
            // camera: CameraConfig {
            // position: (200.0, 39.0, -25.0),
            // yaw: -155.11,
            // pitch: -3.917,
            // },
        }
    }
}
