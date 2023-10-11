use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use thumb_server::{init, resolve, Config};

// *Start -> time:                          [348.92 µs 349.39 µs 349.88 µs]
// *Lazy regex -> time:                     [38.482 µs 38.574 µs 38.692 µs]
// *All regex to Lazy regex -> time:        [7.5027 µs 7.5158 µs 7.5308 µs]
// *Replace clone() result:                 [5.9438 µs 5.9669 µs 5.9946 µs]
fn criterion_benchmark(c: &mut Criterion) {
    // let path = PathBuf::from(
    //     "/Users/nulldata/Documents/projects/rust/thumb_server/tests/testdata/image.jpeg",
    // );
    // let sizes = Vec::
    let conf = Config {
        base_dir: PathBuf::from(
            "/Users/nulldata/Documents/projects/rust/thumb_server/tests/testdata",
        ),
        sizes: ["60x80".to_string(), "300x400".to_string()].to_vec(),
    };
    init(conf);
    c.bench_function("resolve URI", |b| {
        b.iter(|| resolve(black_box("/60x80/image.jpeg")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
