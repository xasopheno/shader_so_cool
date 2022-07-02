use super::utils::audios_and_visuals_from_filename;
use super::utils::write_audio_to_file;
use crate::config::Config;
use crate::error::KintaroError;
use crate::print::PrintState;
use colored::*;
use cradle::prelude::*;
use std::str::FromStr;

pub fn print_audio_and_video(mut config: Config<'static>) -> Result<(), KintaroError> {
    let (audio, visual) = audios_and_visuals_from_filename(config.socool_path)?;

    // let n_frames = (max_frames * 40) + 100;
    let n_frames = 300;

    println!("{}", format!("Number Frames: {}", n_frames).green());

    let mut state = async_std::task::block_on(PrintState::init(&mut config, visual.visual))?;
    for i in 0..n_frames {
        async_std::task::block_on(state.render())
            .unwrap_or_else(|_| panic!("Unable to render frame: {}", i));
    }

    write_audio_to_file(
        audio.as_slice(),
        std::path::PathBuf::from_str("kintaro.wav")
            .expect("unable to create pathbuf for kintaro.wav"),
    )?;

    let command_join_audio_and_video = "ffmpeg -framerate 40 -pattern_type glob -i out/*.png -i kintaro.wav -c:a copy -shortest -c:v libx264 -r 40 -pix_fmt yuv420p out.mov";

    run!(Stdin("yes"), %command_join_audio_and_video);

    Ok(())
}
