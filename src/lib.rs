pub mod application;
pub mod audio;
pub mod camera;
pub mod canvas;
pub mod clock;
pub mod color;
pub mod composition;
pub mod config;
pub mod error;
pub mod frame;
pub mod gen;
pub mod glyphy;
pub mod image_renderer;
pub mod instance;
pub mod op_stream;
pub mod origami;
pub mod print;
pub mod realtime;
pub mod renderable;
pub mod sampler;
pub mod save;
pub mod shader;
pub mod shared;
pub mod surface;
pub mod toy;
pub mod uniforms;
pub mod vertex;

pub use crate::camera::{default::default_cameras, CameraConfig};
pub use crate::color::{
    color_map_from_named_colorsets,
    color_map_from_named_gen_color,
    // helpers::{colorset_from_hex_strings, colorsets_from_vec_hex_strings},
    helpers::*,
    Color,
    ColorMap,
    ColorSet,
    ColorSets,
    NamedValue,
    RandColor,
    RandColorSet,
};
pub use crate::config::Config;
pub use crate::gen::*;
pub use crate::instance::{
    instancer::{Instancer, InstancerInput, InstancerOutput},
    Instance,
};
pub use crate::save::ConfigState;
pub use crate::vertex::shape::{RandIndex, RandPosition, Shape};
pub use kintaro_egui_lib::InstanceMul;
pub use weresocool::error::Error;
