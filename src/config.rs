#[derive(Clone)]
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
    pub camera: CameraConfig,
}

impl Config {
    pub fn new() -> Self {
        Config {
            filename: "kintaro".into(),
            volume: 0.5,
            window_size: (1600, 1000),
            // camera: CameraConfig {
            // position: (0.0, 0.0, 65.0),
            // yaw: -90.0,
            // pitch: 0.0,
            // },
            // camera: CameraConfig {
            // position: (-48.2, -3.5, 16.0),
            // yaw: -50.0,
            // pitch: 8.0,
            // },
            camera: CameraConfig {
                position: (-22.83299, -5.1967072, -35.540905),
                yaw: 16.77102,
                pitch: 5.3154497,
            },
        }
    }
}
