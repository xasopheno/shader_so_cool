use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::config::CameraConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigState {
    pub camera: CameraConfig,
    pub instance_mul: InstanceMul,
}
