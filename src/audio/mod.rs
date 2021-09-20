use rodio::OutputStream;
use std::fs::File;
use std::io::BufReader;

pub fn play_audio(config: &crate::config::Config) -> (OutputStream, rodio::Sink) {
    let filename = format!("./{}.wav", config.filename);
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(&filename).unwrap());
    let stream_handle = stream_handle.play_once(BufReader::new(file)).unwrap();
    stream_handle.pause();
    stream_handle.set_volume(config.volume);
    println!("playing: {}", &filename);
    (stream, stream_handle)
}
