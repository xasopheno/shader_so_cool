use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::CameraState;

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigState {
    pub camera: CameraState,
    pub instance_mul: InstanceMul,
}
