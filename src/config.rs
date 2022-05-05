use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::default::default_cameras;
use crate::instance::instancer::{Instancer, SimpleInstancer};
#[allow(unused_imports)]
use crate::renderable::{
    EventStreamConfig, GlyphyConfig, ImageRendererConfig, OrigamiConfig, RenderableConfig,
    SamplerConfig, ToyConfig,
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

pub fn color_map() -> ColorMap {
    color_map_from_named_colorsets(named_colorsets())
}

#[derive(Clone)]
pub struct FramePass {
    pub output_frame: &'static str,
    pub renderables: Vec<RenderableConfig<'static>>,
}

fn frame_passes() -> Vec<FramePass> {
    vec![
        FramePass {
            output_frame: "frame1",
            renderables: vec![
                RenderableConfig::Toy(ToyConfig {
                    shader_path: "src/origami/shaders/toy3.wgsl",
                }),
                RenderableConfig::EventStreams(EventStreamConfig {
                    socool_path: "kintaro3.socool".to_string(),
                    shader_path: "./src/shader.wgsl",
                    shape: Shape {
                        n_vertices: 50,
                        n_indices: 50,
                        position: Box::new(RandPosition),
                        color: Box::new(color_map()),

                        indices: Box::new(RandIndex),
                    },
                }),
            ],
        },
        FramePass {
            output_frame: "frame2",
            renderables: vec![
                RenderableConfig::Toy(ToyConfig {
                    shader_path: "src/origami/shaders/toy3.wgsl",
                }),
                RenderableConfig::Sampler(SamplerConfig {
                    shader_path: "./src/sampler/sampler_shader.wgsl",
                    input_frame: "frame1",
                }),
                RenderableConfig::Glyphy(GlyphyConfig::GlypyTextConfig {
                    text: vec![("Cool", "#ff2365")],
                    location: (0.7, 0.9),
                    scale: 100.0,
                }),
            ],
        },
        FramePass {
            output_frame: "main",
            renderables: vec![
                // RenderableConfig::ImageRenderer(ImageRendererConfig {
                // image_path: "src/image_renderer/milo.png",
                // }),
                RenderableConfig::Toy(ToyConfig {
                    shader_path: "src/origami/shaders/toy3.wgsl",
                }),
                // RenderableConfig::Origami(OrigamiConfig {
                // shader_path: "./src/origami_shader.wgsl",
                // n_indices: 30,
                // n_vertices: 20,
                // }),
                RenderableConfig::Sampler(SamplerConfig {
                    shader_path: "./src/sampler/sampler_shader.wgsl",
                    input_frame: "frame2",
                }),
            ],
        },
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
            composition_name: "kintaro3",
            frame_passes: frame_passes(),
            instancer: Box::new(SimpleInstancer {}),
            instance_mul,
            accumulation: false,
            volume: 0.20,
            window_size: (2560, 1440),
            cameras,
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
    // Gosh...do I have multiple cameras?
    pub cameras: Vec<CameraConfig>,
    // need to handle opacity correctly
    pub accumulation: bool,
    // move shape, instancer, and instance_mul to appropriate context
    pub instance_mul: InstanceMul,
    pub instancer: Box<dyn Instancer>,
    pub frame_passes: Vec<FramePass>,
}
