use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day23::*;

fn part2_benchmark(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| part_2()));
}

criterion_group!(benches, part2_benchmark);
criterion_main!(benches);
