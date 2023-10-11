use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use thumb_server::return_file;

fn criterion_benchmark(c: &mut Criterion) {
    let path = PathBuf::from(
        "/Users/nulldata/Documents/projects/rust/thumb_server/tests/testdata/image.jpeg",
    );
    c.bench_function("fib 20", |b| b.iter(|| return_file(black_box(&path))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
