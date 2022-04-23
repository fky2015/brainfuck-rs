use std::{collections::VecDeque, fs::File, io::Read};

use brainfuck_rs::{
    io::{StdIn, StdOut},
    Interpreter,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct NullIO {
    pub input: VecDeque<char>,
    input_clone: VecDeque<char>,
}

impl NullIO {
    fn new() -> Self {
        NullIO {
            input: VecDeque::new(),
            input_clone: VecDeque::new(),
        }
    }

    fn load_input(&mut self, input: &str) {
        self.input.clear();
        self.input_clone.clear();
        for c in input.chars() {
            self.input.push_back(c);
            self.input_clone.push_back(c);
        }
    }

    fn reload_input(&mut self) {
        self.input.clear();
        self.input.extend(self.input_clone.iter());
    }
}

impl StdIn for NullIO {
    fn read(&mut self) -> Result<char, std::io::Error> {
        return self.input.pop_front().ok_or(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "No more input",
        ));
    }
}

impl StdOut for NullIO {
    fn print(&mut self, _c: char) {}
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("brainfuck");
    let name = "EasyOpt";
    let mut file = File::open(format!("testing/{}.b", name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    drop(file);

    let mut io = NullIO::new();
    group.sample_size(10);
    group.bench_function(name, |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new(&mut io);
            interpreter.interpret(&contents);
        })
    });
    group.finish();
}

pub fn criterion_benchmark_al(c: &mut Criterion) {
    let mut group = c.benchmark_group("brainfuck");
    let name = "al_count_1";
    let mut file = File::open(format!("testing/{}.b", name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    drop(file);

    let mut io = NullIO::new();
    group.bench_function(name, |b| {
        b.iter(|| {
            let mut interpreter = Interpreter::new(&mut io);
            interpreter.interpret(&contents);
        })
    });
    group.finish();
}
criterion_group!(benches, criterion_benchmark, criterion_benchmark_al);
criterion_main!(benches);
