use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kintaro::application::utils::audios_and_visuals_from_filename;
use kintaro::op_stream::{GetOps, OpReceiver};

fn main() {
    let (_audio, visual) = audios_and_visuals_from_filename("./kintaro3.socool").unwrap();
    let mut r = OpReceiver::init(Some(visual.visual), None);
    let t = 10.0;
    r.get_batch(t, "a");
}
