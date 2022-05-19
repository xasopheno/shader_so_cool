use super::utils::audios_and_visuals_from_frame_passes;
use super::utils::{sum_all_waveforms, write_audio_to_file};
use crate::config::Config;
use crate::error::KintaroError;
use crate::print::PrintState;
use colored::*;
use cradle::prelude::*;
use std::str::FromStr;

pub fn print_audio_and_video(mut config: Config<'static>) -> Result<(), KintaroError> {
    let (audios, visuals_map) = audios_and_visuals_from_frame_passes(&config.frame_passes)?;
    let mut audio: Option<Vec<u8>> = None;
    if audios.len() > 0 {
        let a = sum_all_waveforms(audios);
        audio = Some(a);
    }

    let max_frames = match visuals_map.values().max_by_key(|v| v.length as usize) {
        Some(mf) => mf.length as usize,
        None => 1000,
    };

    let n_frames = (max_frames * 40) + 100;

    println!("{}", format!("Number Frames: {}", n_frames).green());

    let mut state = async_std::task::block_on(PrintState::init(&mut config, &visuals_map))?;
    for i in 0..n_frames {
        async_std::task::block_on(state.render())
            .unwrap_or_else(|_| panic!("Unable to render frame: {}", i));
    }

    if let Some(a) = audio {
        write_audio_to_file(
            a.as_slice(),
            std::path::PathBuf::from_str("kintaro.wav")
                .expect("unable to create pathbuf for kintaro.wav"),
        )?;

        let command_join_audio_and_video = "ffmpeg -framerate 40 -pattern_type glob -i out/*.png -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 40 -pix_fmt yuv420p out.mov";

        run!(Stdin("yes"), %command_join_audio_and_video);
    }

    Ok(())
}
