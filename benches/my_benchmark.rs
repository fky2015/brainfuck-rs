use criterion::{black_box, criterion_group,criterion_main, Criterion };
use brainfuck_rs::{Interpreter, RawIO};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| ))
}
