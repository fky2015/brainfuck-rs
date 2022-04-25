// >	Increment the data pointer (to point to the next cell to the right).
// <	Decrement the data pointer (to point to the next cell to the left).
// +	Increment (increase by one) the byte at the data pointer.
// -	Decrement (decrease by one) the byte at the data pointer.
// .	Output the byte at the data pointer.
// ,	Accept one byte of input, storing its value in the byte at the data pointer.
// [	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ]	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

pub mod io {
    use std::io::Read;

    pub trait StdOut {
        fn print(&mut self, c: char);
    }

    pub trait StdIn {
        fn read(&mut self) -> Result<char, std::io::Error>;
    }

    pub trait StdInOut: StdIn + StdOut {}

    impl<T: StdIn + StdOut> StdInOut for T {}

    pub struct RawIO {}

    impl RawIO {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl StdOut for RawIO {
        fn print(&mut self, c: char) {
            print!("{}", c);
        }
    }

    impl StdIn for RawIO {
        fn read(&mut self) -> Result<char, std::io::Error> {
            std::io::stdin()
                .bytes()
                .next()
                .unwrap()
                .map(|byte| byte as char)
        }
    }
}

mod token {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Token {
        LessThan,
        GreaterThan,
        Plus,
        Minus,
        Dot,
        Comma,
        LeftSquareBracket,
        RightSquareBracket,
        Space,
    }
}

mod bytecode {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Bytecode {
        IncrementPointer,
        DecrementPointer,
        IncrementValue,
        DecrementValue,
        OutputValue,
        InputValue,
        LoopStart { jump_to: usize },
        LoopEnd { jump_to: usize },
    }
}

mod scanner {
    use crate::token::Token;

    // source code -> token
    pub fn scan(input: &str) -> Vec<Token> {
        input
            .chars()
            .into_iter()
            .filter(|c| match c {
                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
                _ => false,
            })
            .map(|c| match c {
                '>' => Token::GreaterThan,
                '<' => Token::LessThan,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '.' => Token::Dot,
                ',' => Token::Comma,
                '[' => Token::LeftSquareBracket,
                ']' => Token::RightSquareBracket,
                _ => panic!("unexpected character: {}", c),
            })
            .collect()
    }

    #[test]
    fn scan_codes() {
        assert_eq!(
            scan("<>+-<>+-"),
            vec![
                Token::LessThan,
                Token::GreaterThan,
                Token::Plus,
                Token::Minus,
                Token::LessThan,
                Token::GreaterThan,
                Token::Plus,
                Token::Minus
            ]
        );
    }
}
mod compiler {

    use super::bytecode::Bytecode;
    use crate::token::Token;

    pub struct Compiler {
        loop_stack: Vec<usize>,
    }

    impl Compiler {
        pub fn new() -> Self {
            Self {
                loop_stack: Vec::new(),
            }
        }

        pub fn compile_bytecode(&mut self, input: Vec<Token>) -> Vec<Bytecode> {
            let mut output = Vec::new();

            input
                .into_iter()
                .filter(|t| t != &Token::Space)
                .enumerate()
                .for_each(|(i, t)| {
                    let next_bytecode = match t {
                        Token::GreaterThan => Bytecode::IncrementPointer,
                        Token::LessThan => Bytecode::DecrementPointer,
                        Token::Plus => Bytecode::IncrementValue,
                        Token::Minus => Bytecode::DecrementValue,
                        Token::Dot => Bytecode::OutputValue,
                        Token::Comma => Bytecode::InputValue,
                        Token::LeftSquareBracket => {
                            self.loop_stack.push(i);
                            Bytecode::LoopStart { jump_to: 0 }
                        }
                        Token::RightSquareBracket => {
                            let loop_start = self.loop_stack.pop().unwrap();
                            // TODO: judge
                            *output.get_mut(loop_start).unwrap() =
                                Bytecode::LoopStart { jump_to: i };
                            Bytecode::LoopEnd {
                                jump_to: loop_start,
                            }
                        }
                        Token::Space => panic!("no space"),
                    };
                    output.push(next_bytecode);
                });

            output
        }
    }

    #[test]
    fn token_to_bytescodes() {
        let mut compiler = Compiler::new();

        let input = vec![Token::GreaterThan, Token::LessThan, Token::Plus];
        let output = compiler.compile_bytecode(input);
        assert_eq!(
            output,
            vec![
                Bytecode::IncrementPointer,
                Bytecode::DecrementPointer,
                Bytecode::IncrementValue
            ]
        )
    }
}

mod vm {
    use crate::{
        bytecode::Bytecode,
        io::{StdIn, StdInOut, StdOut},
    };

    pub struct VirtualMachine<'a> {
        memory: Vec<u8>,
        mem_pointer: usize,
        code_pointer: usize,
        pub io: &'a mut dyn StdInOut,
    }

    impl<'a> VirtualMachine<'a> {
        pub fn new(io: &'a mut dyn StdInOut) -> Self {
            Self {
                memory: vec![0; 3000],
                mem_pointer: 0,
                code_pointer: 0,
                io,
            }
        }

        pub fn run(&mut self, bytecodes: Vec<Bytecode>) {
            loop {
                if let Some(bytecode) = bytecodes.get(self.code_pointer) {
                    // println!(
                    //     "{:?}, mem_pointer: {}, value: {}",
                    //     bytecode, self.mem_pointer, self.memory[self.mem_pointer]
                    // );
                    match bytecode {
                        Bytecode::IncrementPointer => {
                            if self.mem_pointer == self.memory.len() - 1 {
                                self.mem_pointer = 0;
                            } else {
                                self.mem_pointer += 1;
                            }
                        }
                        Bytecode::DecrementPointer => {
                            if self.mem_pointer == 0 {
                                self.mem_pointer = self.memory.len() - 1;
                            } else {
                                self.mem_pointer -= 1;
                            }
                        }
                        Bytecode::IncrementValue => {
                            self.memory[self.mem_pointer] =
                                self.memory[self.mem_pointer].overflowing_add(1).0;
                        }
                        Bytecode::DecrementValue => {
                            self.memory[self.mem_pointer] =
                                self.memory[self.mem_pointer].overflowing_sub(1).0;
                        }
                        Bytecode::OutputValue => {
                            self.io.print(self.memory[self.mem_pointer] as char)
                        }
                        Bytecode::InputValue => {
                            if let Ok(c) = self.io.read() {
                                self.memory[self.mem_pointer] = c as u8;
                            }
                        }
                        Bytecode::LoopStart { jump_to } => {
                            if self.memory[self.mem_pointer] == 0 {
                                self.code_pointer = *jump_to;
                            }
                        }
                        Bytecode::LoopEnd { jump_to } => {
                            if self.memory[self.mem_pointer] != 0 {
                                self.code_pointer = *jump_to;
                            }
                        }
                    }
                } else {
                    // EOF
                    break;
                }

                self.code_pointer += 1;
            }
        }

        pub fn print_memory(&self) {
            println!("{:?}", self.memory);
        }
    }
}

/// # Brainfuck Interpreter
pub mod interpret {
    use crate::{compiler::Compiler, io::StdInOut, scanner::scan, vm::VirtualMachine};

    pub struct Interpreter<'a> {
        vm: VirtualMachine<'a>,
        compiler: Compiler,
    }

    impl<'a> Interpreter<'a> {
        pub fn new(io: &'a mut dyn StdInOut) -> Self {
            Self {
                vm: VirtualMachine::new(io),
                compiler: Compiler::new(),
            }
        }

        pub fn interpret(&mut self, source_code: &str) {
            let tokens = scan(source_code);
            let bytecodes = self.compiler.compile_bytecode(tokens);
            self.vm.run(bytecodes);
        }
    }
}

pub use interpret::Interpreter;
pub use io::RawIO;

pub(crate) mod testing {
    use std::{collections::VecDeque, fs::File, io::Read};

    use crate::{compiler, io, scanner::scan, vm};

    #[macro_export]
    macro_rules! gen_tests {
        ($( $x: expr), *) => {
            $(
                paste!{
                    #[test]
                    fn [<gen_test_ $x:snake>]() {
                        let (source, input, output) = load_test_from_file(&stringify!($x));
                        test_helper(&source, input.as_deref(), output.as_deref());
                    }
                }
            )*
        };
    }

    pub fn test_helper(source_code: &str, input: Option<&str>, output: Option<&str>) {
        let mut io_buffer = TestStdOut::new();

        if let Some(input) = input {
            io_buffer.push_input(input);
        }

        let tokens = scan(source_code);
        let res = compiler::Compiler::new().compile_bytecode(tokens);
        let mut vm = vm::VirtualMachine::new(&mut io_buffer);
        vm.run(res);

        if let Some(output) = output {
            assert_eq!(io_buffer.output, output.chars().collect::<Vec<_>>());
        }
    }

    pub fn load_test_from_file(file_name: &str) -> (String, Option<String>, Option<String>) {
        let mut file = File::open(format!("testing/{}.b", file_name)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let source_code = contents.lines().collect::<String>();

        let stdout = File::open(format!("testing/{}.out", file_name))
            .map(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            })
            .ok();

        let stdin = File::open(format!("testing/{}.in", file_name))
            .map(|mut file| {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents
            })
            .ok();

        (source_code, stdin, stdout)
    }

    #[derive(PartialEq, Debug)]
    pub struct TestStdOut {
        pub input: VecDeque<char>,
        pub output: Vec<char>,
    }

    impl TestStdOut {
        pub fn new() -> Self {
            Self {
                input: VecDeque::new(),
                output: Vec::new(),
            }
        }

        pub fn push_input(&mut self, input: &str) {
            self.input.extend(input.chars());
        }
    }

    impl io::StdOut for TestStdOut {
        fn print(&mut self, c: char) {
            self.output.push(c);
        }
    }

    impl io::StdIn for TestStdOut {
        fn read(&mut self) -> Result<char, std::io::Error> {
            self.input
                .pop_front()
                .ok_or(std::io::Error::new(std::io::ErrorKind::Other, "No input"))
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::testing::{load_test_from_file, test_helper};
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    use paste::paste;

    #[test]
    fn test_io() {
        let source = ",.";
        let input = Some("H");
        let output = Some("H");
        test_helper(source, input, output);
    }

    #[test]
    fn basis_interpreter_test() {
        let source = "++++++++ [>++++++++++++>+++++++++++++<<-] >++++. -. >+++++++. <+. +.";
        let output = "dcode";
        test_helper(source, None, Some(output));
    }

    #[test]
    fn hello_world() {
        let source = "
>++++++++[-<+++++++++>]<.
>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.
>->+++..
+++.
>-.
<<+[>[+>+]>>]<--------------.
>>.
+++.
------.
--------.
>+.
>+.
";
        let output = "Hello World!\n";
        test_helper(source, None, Some(output));
    }

    #[test]
    fn hello_world_2() {
        let source = "
+[>[<->+[>+++>[+++++++++++>][]-[<]>
-]]++++++++++<]>>>>>>----.<<+++.<-.
.+++.<-.>>>.<<.+++.------.>-.<<+.<.
";
        let output = "Hello World!\n";
        test_helper(source, None, Some(output));
    }

    gen_tests![hello_world, Beer, al_count_0, al_count_1, Collatz];

    #[test]
    fn test_examples() {
        let files = vec!["Beer"];

        for file in files {
            let (source, input, output) = load_test_from_file(&file);
            test_helper(&source, input.as_deref(), output.as_deref());
        }
    }
}
