use kintaro_egui_lib::InstanceMul;
use serde::{Deserialize, Serialize};

use crate::camera::default::default_cameras;
use crate::instance::instancer::{Instancer, SimpleInstancer};
use crate::save::ConfigState;
use crate::vertex::shape::{RandIndex, RandPosition, Shape};
#[allow(unused_imports)]
use crate::{color_map_from_named_colorsets, ColorMap, ColorSets};

pub struct ToyConfig {
    shader: wgpu::ShaderModule
    size: (u32, u32), 
    texture_format: wgpu::TextureFormat
}

pub struct ImageRendererConfig {
    image_path: String,
    texture_format: wgpu::TextureFormat
}

pub struct GlyphyConfig {
    text: String,
    texture_format: wgpu::TextureFormat
}

pub struct EventStreamConfig {
    socool_filename: String,
    shader: wgpu::ShaderModule,
    texture_format: wgpu::TextureFormat
}

fn layers() -> Result<(), weresocool::error::Error> {
    // let instance_shader = make_shader(&device, &config.instance_shader)?;
    // let toy_shader = make_shader(&device, &config.toy_shader)?;

    // let toy = crate::toy::setup_toy(
        // &device,
        // toy_shader,
        // size,
        // wgpu::TextureFormat::Bgra8UnormSrgb,
    // );

    // let op_streams = crate::op_stream::OpStream::from_vec_op4d(av);

    // let renderpasses = make_renderpasses(
        // &device,
        // op_streams,
        // &instance_shader,
        // config,
        // wgpu::TextureFormat::Bgra8UnormSrgb,
    // );

    // let image_renderer = pollster::block_on(ImageRenderer::new(
        // &device,
        // &queue,
        // wgpu::TextureFormat::Bgra8UnormSrgb,
    // ));

    // let glyphy = Glyphy::init(
        // &device,
        // wgpu::TextureFormat::Bgra8UnormSrgb,
        // config.text.as_ref().unwrap().to_vec(),
    // )
    // .expect("Unable to setup Glyphy");
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        pub fn named_colorsets<'a>() -> Vec<(&'a str, Vec<&'a str>)> {
            vec![
                ("meg_0311", vec!["#dd1133", "#122333"]),
                ("meg_0321", vec!["#11aa88", "#11a111"]),
                ("meg_0331", vec!["#885533", "#ffaaaa"]),
            ]
        }

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
            instance_shader: "./src/shader.wgsl".into(),
            toy_shader: "./src/toy.wgsl".into(),
            instancer: Box::new(SimpleInstancer {}),
            instance_mul,
            accumulation: false,
            filename: "kintaro".into(),
            volume: 0.20,
            window_size: (2560, 1440),
            cameras,
            text: Some(named_colorsets()),
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
    pub text: Option<Vec<(&'a str, Vec<&'a str>)>>,
    pub filename: String,
    pub volume: f32,
    pub window_size: (u32, u32),
    pub cameras: Vec<CameraConfig>,
    pub accumulation: bool,
    pub shape: Shape,
    pub instance_mul: InstanceMul,
    pub instancer: Box<dyn Instancer>,
    pub instance_shader: String,
    pub toy_shader: String,
}
