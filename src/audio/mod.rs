use rodio::OutputStream;
use std::fs::File;
use std::io::{BufReader, Cursor};

#[allow(unused)]
pub fn play_audio_file(config: &crate::config::Config) -> (OutputStream, rodio::Sink) {
    todo!();
    // let filename = format!("./{}.wav", config.filename);
    // let (stream, stream_handle) = OutputStream::try_default().unwrap();
    // let file = BufReader::new(File::open(&filename).unwrap());
    // let stream_handle = stream_handle.play_once(BufReader::new(file)).unwrap();
    // stream_handle.pause();
    // stream_handle.set_volume(config.volume);
    // println!("playing: {}", &filename);
    // (stream, stream_handle)
}

pub fn setup_audio(config: &crate::config::Config, audio: &Vec<u8>) -> (OutputStream, rodio::Sink) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let stream_handle = stream_handle
        .play_once(Cursor::new(audio.to_owned()))
        .unwrap();
    stream_handle.pause();
    stream_handle.set_volume(config.volume);
    (stream, stream_handle)
}
