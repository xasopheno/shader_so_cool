use rodio::OutputStream;
use std::io::Cursor;

pub fn setup_audio(config: &crate::config::Config, audio: &[u8]) -> (OutputStream, rodio::Sink) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let stream_handle = stream_handle
        .play_once(Cursor::new(audio.to_owned()))
        .unwrap();
    stream_handle.pause();
    stream_handle.set_volume(config.volume);
    (stream, stream_handle)
}
