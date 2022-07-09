use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kintaro::application::utils::audios_and_visuals_from_filename;
use kintaro::op_stream::{GetOps, OpReceiver};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let (_audio, visual) = audios_and_visuals_from_filename("./kintaro3.socool").unwrap();
    let mut r = OpReceiver::init(Some(visual.visual), None);
    let mut t = 100.0;

    c.bench_function("OpReceiver.getBatch", |b| {
        b.iter(|| {
            r.get_batch(t, "a");
            // t+= 3.0
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
