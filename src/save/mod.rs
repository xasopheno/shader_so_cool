use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::config::CameraConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigState {
    pub camera: CameraConfig,
    pub instance_mul: InstanceMul,
}

impl ConfigState {
    pub fn load_saved() -> Result<ConfigState, serde_json::Error> {
        let path = "../kintaro/saved.json";
        let saved_data = std::fs::read_to_string(path).expect("Unable to read file");
        let saved: ConfigState = serde_json::from_str(&saved_data)?;
        Ok(saved)
    }
}
