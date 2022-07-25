use crate::application::{Audio, Visual};
use crate::error::KintaroError;
use std::io::Write;
use weresocool::core::{
    generation::parsed_to_render::AudioVisual,
    generation::{RenderReturn, RenderType},
    interpretable::{InputType, Interpretable},
};
use weresocool::error::Error;

pub fn audios_and_visuals_from_filename(filename: &'static str) -> Result<(Audio, Visual), Error> {
    let result = get_audiovisual_data(filename, true)?;

    let (a, v) = split_audio_visual(result);

    Ok((a, v))
}

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

fn get_audiovisual_data(filename: &str, render_audio: bool) -> Result<AudioVisual, Error> {
    let render_type = if render_audio {
        RenderType::AudioVisual
    } else {
        RenderType::Visual
    };

    if let RenderReturn::AudioVisual(av) = InputType::Filename(filename).make(render_type, None)? {
        Ok(av)
    } else {
        Err(Error::with_msg(format!("Error rendering {}", filename)))
    }
}

pub fn write_audio_to_file(audio: &[u8], filename: std::path::PathBuf) -> Result<(), KintaroError> {
    let mut file = std::fs::File::create(&filename)?;
    file.write_all(audio)?;
    println!("Audio file written: {}", filename.display());
    Ok(())
}

pub fn sum_all_waveforms(mut vec_wav: Vec<Vec<u8>>) -> Vec<u8> {
    // Sort the vectors by length
    sort_vecs(&mut vec_wav);

    // Get the length of the longest vector
    let max_len = vec_wav[0].len();

    let mut result = vec![0; max_len];

    for wav in vec_wav {
        sum_vec(&mut result, &wav[..]);
    }

    result
}

/// Sort a Vec of Vec<u8> by length.
pub fn sort_vecs(vec_wav: &mut Vec<Vec<u8>>) {
    vec_wav.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
}

/// Sum two vectors. Assumes vector a is longer than or of the same length
/// as vector b.
pub fn sum_vec(a: &mut Vec<u8>, b: &[u8]) {
    for (ai, bi) in a.iter_mut().zip(b) {
        *ai += *bi;
    }
}
