use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::default::default_cameras;
use crate::instance::instancer::{Instancer, SimpleInstancer};
use crate::renderable::{
    EventStreamConfig, GlyphyConfig, ImageRendererConfig, RenderableConfig, ToyConfig,
};
use crate::save::ConfigState;
use crate::vertex::shape::{RandIndex, RandPosition, Shape};
#[allow(unused_imports)]
use crate::{color_map_from_named_colorsets, ColorMap, ColorSets};

fn renderable_configs() -> Vec<RenderableConfig<'static>> {
    vec![
        RenderableConfig::Toy(ToyConfig {
            shader_path: "src/toy.wgsl",
            texture_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }),
        RenderableConfig::ImageRenderer(ImageRendererConfig {
            image_path: "src/image_renderer/milo.png",
            texture_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }),
        RenderableConfig::Glyphy(GlyphyConfig {
            text: named_colorsets(),
            texture_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }),
        RenderableConfig::EventStreams(EventStreamConfig {
            filename: "kintaro.socool".to_string(),
            socool_path: "kintaro.socool".to_string(),
            shader_path: "./src/shader.wgsl",
            texture_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }),
    ]
}
pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
    vec![
        ("meg_0311", vec!["#dd1133", "#122333"]),
        ("meg_0321", vec!["#11aa88", "#11a111"]),
        ("meg_0331", vec!["#885533", "#ffaaaa"]),
    ]
}

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
            renderable_configs: renderable_configs(),
            // instance_shader: "./src/shader.wgsl".into(),
            // toy_shader: "./src/toy.wgsl".into(),
            instancer: Box::new(SimpleInstancer {}),
            instance_mul,
            accumulation: false,
            filename: "kintaro",
            volume: 0.20,
            window_size: (2560, 1440),
            cameras,
            // text: Some(named_colorsets()),
            shape: Shape {
                n_vertices: 70,
                n_indices: 70,
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

        let instance_mul = if let Ok(s) = saved {
            if s.is_some() {
                s.unwrap().instance_mul
            } else {
                instance_mul
            }
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
    // pub text: Option<Vec<(&'a str, Vec<&'a str>)>>,
    pub filename: &'a str,
    pub volume: f32,
    pub window_size: (u32, u32),
    pub cameras: Vec<CameraConfig>,
    pub accumulation: bool,
    pub shape: Shape,
    pub instance_mul: InstanceMul,
    pub instancer: Box<dyn Instancer>,
    pub renderable_configs: Vec<RenderableConfig<'a>>,
}
