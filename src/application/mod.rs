pub mod print;
pub mod realtime;
pub mod utils;
use crate::application::print::print_audio_and_video;
use crate::application::realtime::run_realtime;
use crate::config::Config;
use crate::error::KintaroError;

use std::collections::HashMap;
use weresocool::generation::Op4D;

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
