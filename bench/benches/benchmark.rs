use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("fib", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
