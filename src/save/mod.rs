use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::CameraConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigState {
    pub camera: CameraConfig,
    pub instance_mul: InstanceMul,
}

impl ConfigState {
    pub fn load_saved() -> Result<Option<ConfigState>, serde_json::Error> {
        let path = "./save/saved.json";
        if std::path::Path::new(path).is_file() {
            println!("Loading saved state from {}", path);
            let saved_data = std::fs::read_to_string(path).expect("Unable to read file");
            let saved: ConfigState = serde_json::from_str(&saved_data)?;
            Ok(Some(saved))
        } else {
            Ok(None)
        }
    }
}
