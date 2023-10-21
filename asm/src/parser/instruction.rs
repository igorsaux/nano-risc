use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        self,
        complete::{alpha1, char, hex_digit1, line_ending, one_of, space0, space1},
    },
    combinator::{eof, opt, recognize},
    multi::{many0, many1, many_till},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use super::{
    identifier, register, ArgumentToken, ParsingError, ParsingErrorKind, Span, Token, TokenKind,
};

pub fn parse(data: Span) -> IResult<Span, Vec<Token>, ParsingError> {
    alt((self::parse_with_args, self::parse_single))(data)
}

fn parse_single(data: Span) -> IResult<Span, Vec<Token>, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    self::operation_parser(data).map(|(remain, name)| {
        (
            remain,
            vec![Token {
                location,
                kind: TokenKind::Operation { operation: name },
            }],
        )
    })
}

fn parse_with_args(data: Span) -> IResult<Span, Vec<Token>, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    pair(
        self::operation_parser,
        many_till(self::arg_parser, alt((line_ending, eof))),
    )(data)
    .map(|(remain, (name, (mut args, _)))| {
        let mut tokens = vec![Token {
            location,
            kind: TokenKind::Operation { operation: name },
        }];

        tokens.append(&mut args);

        (remain, tokens)
    })
}

fn pin_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    preceded(tag("p"), character::complete::u8)(data).map(|(remain, id)| {
        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Pin { id: id as usize },
                },
            },
        )
    })
}

fn int_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    pair(opt(alt((tag("+"), tag("-")))), character::complete::i32)(data).map(
        |(remain, (sign, value))| {
            let sign = sign
                .map(|value| String::from_utf8(value.to_vec()).unwrap())
                .unwrap_or_else(|| String::from("+"));

            (
                remain,
                Token {
                    location,
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int {
                            value: str::parse(&format!("{sign}{value}")).unwrap(),
                        },
                    },
                },
            )
        },
    )
}

fn hex_int_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    pair(
        opt(alt((tag("+"), tag("-")))),
        preceded(tag("0x"), hex_digit1),
    )(data)
    .map(|(remain, (sign, value))| {
        let sign = sign
            .map(|value| String::from_utf8(value.to_vec()).unwrap())
            .unwrap_or_else(|| String::from("+"));
        let value = String::from_utf8(value.to_vec()).unwrap();

        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Int {
                        value: i32::from_str_radix(&format!("{sign}{value}"), 16).unwrap(),
                    },
                },
            },
        )
    })
}

fn bin_int_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    preceded(
        tag("0b"),
        recognize(many1(terminated(one_of("01"), many0(char('_'))))),
    )(data)
    .map(|(remain, value)| {
        let value = String::from_utf8(value.to_vec()).unwrap();

        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Int {
                        value: i32::from_str_radix(&value, 2).unwrap(),
                    },
                },
            },
        )
    })
}

fn float_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    pair(
        opt(alt((tag("+"), tag("-")))),
        recognize(separated_pair(
            character::complete::i32,
            tag("."),
            character::complete::i32,
        )),
    )(data)
    .map(|(remain, (sign, value))| {
        let sign = sign
            .map(|value| String::from_utf8(value.to_vec()).unwrap())
            .unwrap_or_else(|| String::from("+"));
        let value = String::from_utf8(value.to_vec()).unwrap();

        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Float {
                        value: f32::from_str(&format!("{sign}{value}")).unwrap(),
                    },
                },
            },
        )
    })
}

fn string_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    delimited(tag("\""), take_till(|c| c == b'"'), tag("\""))(data).map(|(remain, value)| {
        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::String {
                        value: String::from_utf8(value.to_vec()).unwrap(),
                    },
                },
            },
        )
    })
}

fn label_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    identifier(data).map(|(remain, name)| {
        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Label { name },
                },
            },
        )
    })
}

fn register_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    register::parse(data).map(|(remain, kind)| {
        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Register { register: kind },
                },
            },
        )
    })
}

fn constant_arg(data: Span) -> IResult<Span, Token, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    preceded(tag("."), identifier)(data).map(|(remain, name)| {
        (
            remain,
            Token {
                location,
                kind: TokenKind::Argument {
                    argument: ArgumentToken::Constant { name },
                },
            },
        )
    })
}

fn arg_parser(data: Span) -> IResult<Span, Token, ParsingError> {
    terminated(
        alt((
            self::register_arg,
            self::pin_arg,
            self::bin_int_arg,
            self::hex_int_arg,
            self::float_arg,
            self::int_arg,
            self::string_arg,
            self::constant_arg,
            self::label_arg,
        )),
        space0,
    )(data)
    .map(|(remain, arg)| (remain, arg))
    .map_err(|err| {
        ParsingError::from_nom_error(
            String::from(
                "Expected a register, pin, integer, float, string, constant or label argument",
            ),
            err,
            ParsingErrorKind::InvalidArgument,
        )
    })
}

fn operation_parser(data: Span) -> IResult<Span, String, ParsingError> {
    terminated(alpha1, alt((line_ending, eof, space1)))(data).map(|(remain, name)| {
        (
            remain,
            String::from_utf8(name.to_vec()).unwrap().to_lowercase(),
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::parser::{ArgumentToken, Span, Token, TokenKind};
    use nano_risc_arch::{Location, RegisterKind, RegisterMode, SourceUnit};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_single_instruction() {
        let unit = SourceUnit::new_anonymous("add".as_bytes().to_vec());
        let instruction = super::parse(Span::new_extra(unit.data(), unit.clone()));

        assert_eq!(
            instruction.map(|(_, token)| token),
            Ok(vec![Token {
                location: Location {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                kind: TokenKind::Operation {
                    operation: String::from("add"),
                }
            }])
        )
    }

    #[test]
    fn parse_instruction_with_args() {
        let unit = SourceUnit::new_anonymous(
            "add $r1 p4 78 -99 0xFF -0xDD 0b0101 12.66 -4.12 \"Hello, world!\" start .data"
                .as_bytes()
                .to_vec(),
        );
        let instruction = super::parse(Span::new_extra(unit.data(), unit.clone()));

        assert_eq!(
            instruction.map(|(_, token)| token),
            Ok(vec![
                Token {
                    location: Location {
                        line: 1,
                        column: 1,
                        offset: 0
                    },
                    kind: TokenKind::Operation {
                        operation: String::from("add"),
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 5,
                        offset: 4
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Register {
                            register: RegisterKind::Regular {
                                id: 1,
                                mode: RegisterMode::Direct
                            }
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 9,
                        offset: 8
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Pin { id: 4 }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 12,
                        offset: 11
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: 78 }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 15,
                        offset: 14
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: -99 }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 19,
                        offset: 18
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: 0xFF }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 24,
                        offset: 23
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: -0xDD }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 30,
                        offset: 29
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Int { value: 0b0101 }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 37,
                        offset: 36
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Float { value: 12.66 }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 43,
                        offset: 42
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Float { value: -4.12 }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 49,
                        offset: 48
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::String {
                            value: String::from("Hello, world!")
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 65,
                        offset: 64
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Label {
                            name: String::from("start")
                        }
                    }
                },
                Token {
                    location: Location {
                        line: 1,
                        column: 71,
                        offset: 70
                    },
                    kind: TokenKind::Argument {
                        argument: ArgumentToken::Constant {
                            name: String::from("data")
                        }
                    }
                },
            ])
        );
    }
}
