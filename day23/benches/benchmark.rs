use day23::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn part2_benchmark(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| part_2()));
}


criterion_group!(benches, part2_benchmark);
criterion_main!(benches);

