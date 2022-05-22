use std::io::{Read, Write};

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Brainfuck-rs CLI")
        .version("0.1.0")
        .author("Feng Kaiyu <loveress01@outlook.com>")
        .about("A Brainfuck interpreter written in Rust")
        .arg(
            Arg::new("INPUT")
                .help("Sets the input file to use")
                .index(1),
        )
        .get_matches();

    let mut rawio = brainfuck_rs::io::RawIO::new();
    let mut interpreter = brainfuck_rs::Interpreter::new(&mut rawio);
    if let Some(input_file) = matches.value_of("INPUT") {
        // Interpreter mode.
        let mut input_file = std::fs::File::open(input_file).unwrap();
        let mut content = String::new();
        input_file.read_to_string(&mut content).unwrap();
        interpreter.interpret(&content);
    } else {
        // REPL mode.
        loop {
            print!(">>>");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            interpreter.interpret(&input);
        }
    }
}
