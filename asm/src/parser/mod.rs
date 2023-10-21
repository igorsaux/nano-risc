mod argument_token;
mod comment;
mod instruction;
mod label;
mod parsing_error;
mod parsing_error_kind;
mod register;
mod token;
mod token_kind;

pub use argument_token::ArgumentToken;
use nano_risc_arch::SourceUnit;
use nom::{
    branch::alt, character::complete::multispace0, combinator::eof, multi::many_till,
    sequence::preceded, Finish,
};
use nom_locate::LocatedSpan;
pub use parsing_error::ParsingError;
pub use parsing_error_kind::ParsingErrorKind;
pub use token::Token;
pub use token_kind::TokenKind;
pub(crate) type Span<'a> = LocatedSpan<&'a [u8], SourceUnit>;

pub fn parse(unit: &SourceUnit) -> Result<Vec<Token>, ParsingError> {
    parse_inner(unit)
}

fn parse_inner(unit: &SourceUnit) -> Result<Vec<Token>, ParsingError> {
    many_till(
        alt((
            preceded(multispace0, label::parse),
            preceded(multispace0, instruction::parse),
            preceded(multispace0, comment::parse),
        )),
        alt((eof, preceded(multispace0, eof))),
    )(Span::new_extra(unit.data(), unit.clone()))
    .finish()
    .map(|(_, (tokens, _))| tokens.concat())
    .map_err(|err| -> ParsingError {
        ParsingError::wrap(
            String::from("Expected label, instruction or comment"),
            err.location(),
            ParsingErrorKind::Unknown,
            err.inner().cloned().unwrap(),
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::parser::{self, ArgumentToken, Token, TokenKind};
    use nano_risc_arch::{Location, RegisterKind, RegisterMode, SourceUnit};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse() {
        let src = r#"
# A basic program
start:
add $r0 1 0
sub $r5 $r0

# Jump
jmp start

# Print stack pointer and program counter
dbg $sp
dbg $pc

mov $r0 %r1
mov $r0 @r2
"#;
        let tokens = parser::parse(&SourceUnit::new_anonymous(src.as_bytes().to_vec()));

        assert_eq!(
            tokens,
            Ok(vec![
                Token {
                    location: Location {
                        line: 2,
                        column: 1,
                        offset: 1
                    },
                    kind: TokenKind::Comment {
                        text: String::from(" A basic program")
                    }
                },
                Token {
                    location: Location {
                        line: 3,
                        column: 1,
                        offset: 19
                    },
                    kind: TokenKind::Label {
                        name: String::from("start")
                    }
                },
                Token {
                    location: Location {
                        line: 4,
                        column: 1,
                        offset: 26
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("add"),
                    }
                },
                Token {
                    location: Location {
                        line: 4,
                        column: 5,
                        offset: 30
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 0,
                                mode: RegisterMode::Direct
                            }
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 4,
                        column: 9,
                        offset: 34
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: 1 }
                    }
                },
                Token {
                    location: Location {
                        line: 4,
                        column: 11,
                        offset: 36
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: 0 }
                    }
                },
                Token {
                    location: Location {
                        line: 5,
                        column: 1,
                        offset: 38
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("sub"),
                    }
                },
                Token {
                    location: Location {
                        line: 5,
                        column: 5,
                        offset: 42
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 5,
                                mode: RegisterMode::Direct
                            }
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 5,
                        column: 9,
                        offset: 46
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 0,
                                mode: RegisterMode::Direct
                            }
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 7,
                        column: 1,
                        offset: 51
                    },
                    kind: TokenKind::Comment {
                        text: String::from(" Jump")
                    }
                },
                Token {
                    location: Location {
                        line: 8,
                        column: 1,
                        offset: 58
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("jmp"),
                    }
                },
                Token {
                    location: Location {
                        line: 8,
                        column: 5,
                        offset: 62
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Label {
                            name: String::from("start")
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 10,
                        column: 1,
                        offset: 69
                    },
                    kind: TokenKind::Comment {
                        text: String::from(" Print stack pointer and program counter")
                    }
                },
                Token {
                    location: Location {
                        line: 11,
                        column: 1,
                        offset: 111
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("dbg"),
                    }
                },
                Token {
                    location: Location {
                        line: 11,
                        column: 5,
                        offset: 115
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::StackPointer
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 12,
                        column: 1,
                        offset: 119
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("dbg"),
                    }
                },
                Token {
                    location: Location {
                        line: 12,
                        column: 5,
                        offset: 123
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::ProgramCounter
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 14,
                        column: 1,
                        offset: 128,
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("mov")
                    },
                },
                Token {
                    location: Location {
                        line: 14,
                        column: 5,
                        offset: 132,
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 0,
                                mode: RegisterMode::Direct,
                            },
                        },
                    },
                },
                Token {
                    location: Location {
                        line: 14,
                        column: 9,
                        offset: 136,
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 1,
                                mode: RegisterMode::Indirect,
                            },
                        },
                    },
                },
                Token {
                    location: Location {
                        line: 15,
                        column: 1,
                        offset: 140,
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("mov")
                    },
                },
                Token {
                    location: Location {
                        line: 15,
                        column: 5,
                        offset: 144,
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 0,
                                mode: RegisterMode::Direct,
                            },
                        },
                    },
                },
                Token {
                    location: Location {
                        line: 15,
                        column: 9,
                        offset: 148,
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 2,
                                mode: RegisterMode::Address,
                            },
                        },
                    },
                },
            ])
        )
    }
}
