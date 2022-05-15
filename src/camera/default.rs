use super::CameraConfig;

pub fn default_cameras(
    mut vec_saved_cameras: Vec<CameraConfig>,
    offset: Option<(f32, f32, f32)>,
) -> Vec<CameraConfig> {
    let offset = if let Some(o) = offset {
        o
    } else {
        (0.0, 0.0, 0.0)
    };
    vec_saved_cameras.append(&mut vec![
        CameraConfig {
            position: (310.0 + offset.0, 83.0 + offset.1, 77.0 + offset.2),
            yaw: -142.0,
            pitch: 1.77,
        },
        CameraConfig {
            position: (-218.0 + offset.0, -40.0 + offset.1, -89.0 + offset.2),
            yaw: -4.0,
            pitch: 31.8,
        },
        CameraConfig {
            position: (-116.2 + offset.0, 36.0 + offset.1, 106.0 + offset.2),
            yaw: -56.11,
            pitch: 5.917,
        },
        CameraConfig {
            position: (0.0 + offset.0, 80.0 + offset.1, 400.0 + offset.2),
            yaw: -90.0,
            pitch: 11.0,
        },
        CameraConfig {
            position: (0.0 + offset.0, 670.0 + offset.1, -226.0 + offset.2),
            yaw: 0.0,
            pitch: -90.0,
        },
    ]);
    vec_saved_cameras
}
