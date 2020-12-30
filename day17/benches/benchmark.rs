use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day17::*;
use std::fs;

fn part2_benchmark(c: &mut Criterion) {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    c.bench_function("part2", |b| b.iter(|| part2(&contents)));
    c.bench_function("bv_part2", |b| b.iter(|| bv_part2(&contents)));
}

criterion_group!(benches, part2_benchmark);
criterion_main!(benches);
