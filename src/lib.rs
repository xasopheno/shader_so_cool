pub mod application;
pub mod audio;
pub mod camera;
pub mod canvas;
pub mod clock;
pub mod color;
pub mod composition;
pub mod config;
pub mod gen;
pub mod instance;
pub mod op_stream;
pub mod print;
pub mod realtime;
pub mod save;
pub mod shader;
pub mod shared;
pub mod toy;
pub mod uniforms;
pub mod vertex;

pub use crate::camera::default::default_cameras;
pub use crate::color::{helpers::*, Color, ColorMap, ColorSet, ColorSets};
pub use crate::config::{CameraConfig, Config};
pub use crate::instance::{
    instancer::{Instancer, InstancerInput, InstancerOutput},
    Instance,
};
pub use crate::save::ConfigState;
pub use crate::vertex::shape::{RandIndex, RandPosition, Shape};
pub use kintaro_egui_lib::InstanceMul;
pub use weresocool::error::Error;
