mod comment;
mod instruction;
mod label;
mod register;

use arch::Assembly;
use nom::{
    branch::alt, character::complete::multispace0, combinator::eof, multi::many_till,
    sequence::preceded,
};

#[derive(Debug, Clone)]
pub struct Parser {
    data: Vec<u8>,
}

impl Parser {
    pub fn new_string(data: String) -> Self {
        Self {
            data: data.into_bytes(),
        }
    }

    pub fn new_bytes(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn parse(self) -> Result<Assembly, ()> {
        let data = &self.data;

        many_till(
            alt((
                preceded(multispace0, label::parse),
                preceded(multispace0, instruction::parse),
                preceded(multispace0, comment::parse),
            )),
            alt((eof, preceded(multispace0, eof))),
        )(data)
        .map(|(_, (tokens, _))| Assembly { tokens })
        .map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use arch::{Argument, Assembly, Operation, RegisterKind, Token};

    use crate::Parser;

    #[test]
    fn parse() {
        let src = r#"
# A basic program
start:
add $r0 1
sub $r5 $r0

# Jump
jmp start

# Print stack pointer and program counter
dbg $sp
dbg $pc
"#;
        let parser = Parser::new_string(src.to_string());
        let assembly = parser.parse();

        assert_eq!(
            assembly,
            Ok(Assembly {
                tokens: vec![
                    Token::Comment {
                        text: String::from(" A basic program")
                    },
                    Token::Label {
                        name: String::from("start")
                    },
                    Token::Instruction {
                        operation: Operation::Add,
                        args: vec![
                            Argument::Register {
                                register: RegisterKind::Regular { id: 0 }
                            },
                            Argument::Int { value: 1 }
                        ]
                    },
                    Token::Instruction {
                        operation: Operation::Sub,
                        args: vec![
                            Argument::Register {
                                register: RegisterKind::Regular { id: 5 }
                            },
                            Argument::Register {
                                register: RegisterKind::Regular { id: 0 }
                            }
                        ]
                    },
                    Token::Comment {
                        text: String::from(" Jump")
                    },
                    Token::Instruction {
                        operation: Operation::Jmp,
                        args: vec![Argument::Label {
                            name: String::from("start")
                        }]
                    },
                    Token::Comment {
                        text: String::from(" Print stack pointer and program counter")
                    },
                    Token::Instruction {
                        operation: Operation::Dbg,
                        args: vec![Argument::Register {
                            register: RegisterKind::StackPointer
                        }]
                    },
                    Token::Instruction {
                        operation: Operation::Dbg,
                        args: vec![Argument::Register {
                            register: RegisterKind::ProgramCounter
                        }]
                    }
                ]
            })
        )
    }
}
