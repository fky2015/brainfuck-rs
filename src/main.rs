// >	Increment the data pointer (to point to the next cell to the right).
// <	Decrement the data pointer (to point to the next cell to the left).
// +	Increment (increase by one) the byte at the data pointer.
// -	Decrement (decrease by one) the byte at the data pointer.
// .	Output the byte at the data pointer.
// ,	Accept one byte of input, storing its value in the byte at the data pointer.
// [	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ]	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

use crate::scanner::scan;

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
    use nom::character::complete::one_of;
    use nom::multi::many0;
    use nom::IResult;

    // source code -> token
    pub fn scan(input: &str) -> IResult<&str, Vec<Token>> {
        let is_code = one_of("<>+-.,[] ");

        let res = many0(is_code)(input)?;

        Ok((
            res.0,
            res.1
                .into_iter()
                .map(|c| match c {
                    '>' => Token::GreaterThan,
                    '<' => Token::LessThan,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '.' => Token::Dot,
                    ',' => Token::Comma,
                    '[' => Token::LeftSquareBracket,
                    ']' => Token::RightSquareBracket,
                    ' ' => Token::Space,
                    _ => panic!("unexpected character: {}", c),
                })
                .collect(),
        ))
    }

    #[test]
    fn scan_codes() {
        assert_eq!(
            scan("<>+-"),
            Ok((
                "",
                vec![
                    Token::LessThan,
                    Token::GreaterThan,
                    Token::Plus,
                    Token::Minus
                ]
            ))
        );
    }
}

mod compiler {
    use crate::token::Token;

    // TOKEN -> BYTECODE
    use super::bytecode::Bytecode;
    use nom::{
        bytes::complete::{tag, take_while_m_n},
        combinator::map_res,
        multi::many_m_n,
        sequence::tuple,
        IResult,
    };

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
            let mut loop_jump_to = 0;
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
}

mod vm {
    use crate::bytecode::Bytecode;

    pub struct VirtualMachine {
        memory: Vec<u8>,
        mem_pointer: usize,
        code_pointer: usize,
    }

    impl VirtualMachine {
        pub fn new() -> Self {
            Self {
                memory: vec![0; 30],
                mem_pointer: 0,
                code_pointer: 0,
            }
        }
        pub fn run(&mut self, bytecodes: Vec<Bytecode>) {
            loop {
                if let Some(bytecode) = bytecodes.get(self.code_pointer) {
                    match bytecode {
                        Bytecode::IncrementPointer => {
                            self.mem_pointer += 1;
                        }
                        Bytecode::DecrementPointer => {
                            self.mem_pointer -= 1;
                        }
                        Bytecode::IncrementValue => {
                            self.memory[self.mem_pointer] += 1;
                        }
                        Bytecode::DecrementValue => {
                            self.memory[self.mem_pointer] -= 1;
                        }
                        Bytecode::OutputValue => {
                            print!("{}", self.memory[self.mem_pointer] as char);
                        }
                        Bytecode::InputValue => {
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
                                println!("jump to: {}", jump_to);
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

#[test]
fn test() {
    let input = "++++++++ [>++++++++++++>+++++++++++++<<-] >++++. -. >+++++++. <+. +.";

    let res = scan(input);
    let res = compiler::Compiler::new().compile_bytecode(res.unwrap().1);
    println!("{:?}", res);
    let mut vm = vm::VirtualMachine::new();
    vm.run(res);

    vm.print_memory();
}
