// >	Increment the data pointer (to point to the next cell to the right).
// <	Decrement the data pointer (to point to the next cell to the left).
// +	Increment (increase by one) the byte at the data pointer.
// -	Decrement (decrease by one) the byte at the data pointer.
// .	Output the byte at the data pointer.
// ,	Accept one byte of input, storing its value in the byte at the data pointer.
// [	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ]	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

mod io {

    pub trait StdOut {
        fn print(&mut self, c: char);
    }

    pub struct OutputBuffer {}

    impl OutputBuffer {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl StdOut for OutputBuffer {
        fn print(&mut self, c: char) {
            println!("{}", c);
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
    use crate::{bytecode::Bytecode, io::StdOut};

    pub struct VirtualMachine<'a> {
        memory: Vec<u8>,
        mem_pointer: usize,
        code_pointer: usize,
        pub stdout: &'a mut dyn StdOut,
    }

    impl<'a> VirtualMachine<'a> {
        pub fn new(stdout: &'a mut dyn StdOut) -> Self {
            Self {
                memory: vec![0; 30],
                mem_pointer: 0,
                code_pointer: 0,
                stdout,
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
                            self.stdout.print(self.memory[self.mem_pointer] as char)
                        }
                        Bytecode::InputValue => {
                            // TODO:
                            // self.memory[self.pointer] = std::io::stdin().bytes().next().unwrap().unwrap()
                            //     as u8;
                            println!("input value");
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

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    pub struct TestStdOut {
        pub buffer: Vec<char>,
    }

    impl TestStdOut {
        pub fn new() -> Self {
            Self { buffer: Vec::new() }
        }
    }

    impl io::StdOut for TestStdOut {
        fn print(&mut self, c: char) {
            self.buffer.push(c);
        }
    }

    #[test]
    fn basis_interpreter_test() {
        let mut stdout = TestStdOut::new();
        let input = "++++++++ [>++++++++++++>+++++++++++++<<-] >++++. -. >+++++++. <+. +.";

        let tokens = scan(input);
        let res = compiler::Compiler::new().compile_bytecode(tokens);
        let mut vm = vm::VirtualMachine::new(&mut stdout);
        vm.run(res);

        assert_eq!(stdout.buffer, vec!['d', 'c', 'o', 'd', 'e']);
    }

    #[test]
    fn hello_world() {
        let mut stdout = TestStdOut::new();
        let input = "
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

        let tokens = scan(input);
        // println!("{:?}", tokens);
        let bytecodes = compiler::Compiler::new().compile_bytecode(tokens);
        // println!("{:?}", bytecodes);
        let mut vm = vm::VirtualMachine::new(&mut stdout);
        vm.run(bytecodes);

        assert_eq!(stdout.buffer, "Hello World!\n".chars().collect::<Vec<_>>());
    }

    #[test]
    fn hello_world_2() {
        let mut stdout = TestStdOut::new();
        let input = "
+[>[<->+[>+++>[+++++++++++>][]-[<]>
-]]++++++++++<]>>>>>>----.<<+++.<-.
.+++.<-.>>>.<<.+++.------.>-.<<+.<.
";

        let tokens = scan(input);
        println!("{:?}", tokens);
        let bytecodes = compiler::Compiler::new().compile_bytecode(tokens);
        println!("{:?}", bytecodes);
        let mut vm = vm::VirtualMachine::new(&mut stdout);
        vm.run(bytecodes);

        assert_eq!(stdout.buffer, "Hello World!\n".chars().collect::<Vec<_>>());
    }
}
