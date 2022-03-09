use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::default::default_cameras;
use crate::instance::instancer::{Instancer, SimpleInstancer};
#[allow(unused_imports)]
use crate::renderable::{
    EventStreamConfig, GlyphyConfig, ImageRendererConfig, OrigamiConfig, RenderableConfig,
    ToyConfig,
};
use crate::save::ConfigState;
use crate::vertex::shape::{RandIndex, RandPosition, Shape};
#[allow(unused_imports)]
use crate::{color_map_from_named_colorsets, ColorMap, ColorSets};

pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("a", vec!["#dd1133", "#030303"]),
        ("b", vec!["#2339e3", "#303030"]),
        ("c", vec!["#744253", "#ccddaa"]),
        ("d", vec!["#887880", "#facba2"]),
        ("e", vec!["#63474D", "#adc37a"]),
        ("f", vec!["#683347", "#faed00"]),
        ("g", vec!["#1A3A3A", "#fadf23"]),
        ("h", vec!["#383F51", "#dadfea"]),
        ("i", vec!["#cc2277", "#232423"]),
        ("j", vec!["#ee4499", "#3f2527"]),
    ]
}

fn renderable_configs() -> Vec<RenderableConfig<'static>> {
    vec![
        // RenderableConfig::ImageRenderer(ImageRendererConfig {
        // image_path: "src/image_renderer/milo.png",
        // }),
        // RenderableConfig::Toy(ToyConfig {
        // shader_path: "src/toy.wgsl",
        // }),
        // RenderableConfig::EventStreams(EventStreamConfig {
        // socool_path: "kintaro.socool".to_string(),
        // shader_path: "./src/shader.wgsl",
        // }),
        // RenderableConfig::Toy(ToyConfig {
        // shader_path: "src/toy2.wgsl",
        // }),
        // RenderableConfig::Glyphy(GlyphyConfig::GlyphyNamedColorSetConfig {
        // text: named_colorsets(),
        // location: (0.05, 0.9),
        // scale: 50.0,
        // }),
        // RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
        // text: vec![("Mimpy", "#ff2323")],
        // location: (0.3, 0.2),
        // scale: 200.0,
        // }),
        RenderableConfig::Toy(ToyConfig {
            shader_path: "src/origami/shaders/toy3.wgsl",
        }),
        RenderableConfig::EventStreams(EventStreamConfig {
            socool_path: "kintaro.socool".to_string(),
            shader_path: "./src/shader.wgsl",
        }),
        // RenderableConfig::Origami(OrigamiConfig {
        // shader_path: "./src/origami_shader.wgsl",
        // n_indices: 30,
        // n_vertices: 20,
        // }),
        RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
            text: vec![("Cool", "#ff2365")],
            location: (0.7, 0.9),
            scale: 170.0,
        }),
    ]
}

// pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
// vec![
// ("meg_0311", vec!["#dd1133", "#122333"]),
// ("meg_0321", vec!["#11aa88", "#11a111"]),
// ("meg_0331", vec!["#885533", "#ffaaaa"]),
// ]
// }

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        let instance_mul = InstanceMul {
            x: 9.0,
            y: 19.0,
            z: 1.0,
            life: 2.0,
            size: 23.0,
            length: 1.0,
        };
        let (cameras, instance_mul) = Config::handle_save(instance_mul);
        Config {
            composition_name: "kintaro",
            renderable_configs: renderable_configs(),
            instancer: Box::new(SimpleInstancer {}),
            instance_mul,
            accumulation: false,
            volume: 0.20,
            window_size: (2560, 1440),
            cameras,
            shape: Shape {
                n_vertices: 50,
                n_indices: 50,
                position: Box::new(RandPosition),
                color: Box::new(color_map_from_named_colorsets(named_colorsets())),

                indices: Box::new(RandIndex),
            },
        }
    }
}

impl<'a> Config<'a> {
    pub fn handle_save(instance_mul: InstanceMul) -> (Vec<CameraConfig>, InstanceMul) {
        let saved = ConfigState::load_saved();
        let cameras = default_cameras(
            if let Ok(ref s) = saved {
                if s.is_some() {
                    vec![s.as_ref().unwrap().camera]
                } else {
                    vec![]
                }
            } else {
                vec![]
            },
            Some((0.0, 20.0, 0.0)),
        );

        let instance_mul = if let Ok(Some(found)) = saved {
            found.instance_mul
        } else {
            instance_mul
        };
        (cameras, instance_mul)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub position: (f32, f32, f32),
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone)]
pub struct Config<'a> {
    pub composition_name: &'a str,
    pub volume: f32,
    pub window_size: (u32, u32),
    pub cameras: Vec<CameraConfig>,
    pub accumulation: bool,
    pub shape: Shape,
    pub instance_mul: InstanceMul,
    pub instancer: Box<dyn Instancer>,
    pub renderable_configs: Vec<RenderableConfig<'a>>,
}
