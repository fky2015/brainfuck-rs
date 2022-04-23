use std::{fs::File, io::Read};

use brainfuck_rs::{Interpreter, RawIO};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("brainfuck");
    group.sample_size(10);
    group.bench_function("EasyOpt", |b| {
        b.iter(|| {
            let mut io = RawIO::new();
            let mut interpreter = Interpreter::new(&mut io);
            let file_name = "EasyOpt";
            let mut file = File::open(format!("testing/{}.b", file_name)).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            interpreter.interpret(&contents);
        })
    });
    group.finish();
}

pub fn criterion_benchmark_beer(c: &mut Criterion) {
    let mut group = c.benchmark_group("brainfuck");
    group.bench_function("Beer", |b| {
        b.iter(|| {
            let mut io = RawIO::new();
            let mut interpreter = Interpreter::new(&mut io);
            let file_name = "Beer";
            let mut file = File::open(format!("testing/{}.b", file_name)).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            interpreter.interpret(&contents);
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_beer);
criterion_main!(benches);
