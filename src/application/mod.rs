pub mod print;
pub mod realtime;
pub mod utils;
use crate::application::print::print_audio_and_video;
use crate::application::realtime::run_realtime;
use crate::config::{Config, FramePass};
use crate::error::KintaroError;
use crate::renderable::RenderableConfig;
use std::collections::HashMap;
use weresocool::{
    error::Error,
    generation::parsed_to_render::AudioVisual,
    generation::{Op4D, RenderReturn, RenderType},
    interpretable::{InputType, Interpretable},
};

pub type VisualsMap = HashMap<String, Visual>;

#[derive(Clone, Debug)]
/// AudioVisual is the datatype for audiovisualization
pub struct Visual {
    /// Composition name
    pub name: String,
    /// length of seconds of composition
    pub length: f32,
    /// visual data
    pub visual: Vec<Op4D>,
}

pub type Audio = Vec<u8>;

/// Sum a Vec<Vec<u8> to a single Vec<u8>.
fn split_audio_visual(av: AudioVisual) -> (Audio, Visual) {
    (
        av.audio,
        Visual {
            name: av.name.clone(),
            length: av.length,
            visual: av.visual,
        },
    )
}

pub fn run(filename: &str, config: Config<'static>) -> Result<(), KintaroError> {
    println!("preparing for audiovisualization: {}", &filename);

    if std::env::args().any(|x| x == "--print") {
        print_audio_and_video(config)?;
    } else {
        println!("****REALTIME****");
        run_realtime(config)?;
    }
    Ok(())
}

pub fn audios_and_visuals_from_frame_passes(
    frame_passes: &Vec<FramePass>,
) -> Result<(Vec<Audio>, VisualsMap), Error> {
    let mut visuals_map: VisualsMap = HashMap::new();
    let mut audios: Vec<Audio> = vec![];

    for c in frame_passes.iter().flat_map(|c| &c.renderables) {
        if let RenderableConfig::EventStreams(e) = c {
            let result = get_audiovisual_data(&e.socool_path)?;

            let (a, v) = split_audio_visual(result);
            audios.push(a);
            visuals_map.insert(e.socool_path.to_string(), v);
        }
    }

    Ok((audios, visuals_map))
}

fn get_audiovisual_data(filename: &str) -> Result<AudioVisual, Error> {
    if let RenderReturn::AudioVisual(av) =
        InputType::Filename(filename).make(RenderType::AudioVisual, None)?
    {
        Ok(av)
    } else {
        Err(Error::with_msg(format!("Error rendering {}", filename)))
    }
}
