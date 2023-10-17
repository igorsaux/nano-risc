use std::str::FromStr;

use arch::{Operation, Token};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::{
        self,
        complete::{alpha1, hex_digit1, line_ending, space0, space1},
    },
    combinator::{eof, opt, recognize},
    multi::many_till,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

pub fn parse(source: &[u8]) -> IResult<&[u8], Token> {
    alt((self::parse_with_args, self::parse_single))(source)
}

fn parse_single(source: &[u8]) -> IResult<&[u8], Token> {
    self::operation_parser(source).map(|(remain, name)| {
        (
            remain,
            Token::Instruction {
                operation: Operation::from_str(&name).unwrap(),
                args: Vec::new(),
            },
        )
    })
}

fn parse_with_args(source: &[u8]) -> IResult<&[u8], Token> {
    pair(
        self::operation_parser,
        many_till(self::arg_parser, alt((line_ending, eof))),
    )(source)
    .map(|(remain, (name, (args, _)))| {
        (
            remain,
            Token::Instruction {
                operation: Operation::from_str(&name).unwrap(),
                args: args.to_vec(),
            },
        )
    })
}

fn register_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    preceded(tag("r"), character::complete::u8)(source)
        .map(|(remain, id)| (remain, arch::Argument::Register { id: id as usize }))
}

fn pin_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    preceded(tag("p"), character::complete::u8)(source)
        .map(|(remain, id)| (remain, arch::Argument::Pin { id: id as usize }))
}

fn int_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    pair(opt(alt((tag("+"), tag("-")))), character::complete::i32)(source).map(
        |(remain, (sign, value))| {
            let sign = sign
                .map(|value| String::from_utf8(value.to_vec()).unwrap())
                .unwrap_or_else(|| String::from("+"));

            (
                remain,
                arch::Argument::Int {
                    value: str::parse(&format!("{sign}{value}")).unwrap(),
                },
            )
        },
    )
}

fn hex_int_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    pair(
        opt(alt((tag("+"), tag("-")))),
        preceded(tag("0x"), hex_digit1),
    )(source)
    .map(|(remain, (sign, value))| {
        let sign = sign
            .map(|value| String::from_utf8(value.to_vec()).unwrap())
            .unwrap_or_else(|| String::from("+"));
        let value = String::from_utf8(value.to_vec()).unwrap();

        (
            remain,
            arch::Argument::Int {
                value: i32::from_str_radix(&format!("{sign}{value}"), 16).unwrap(),
            },
        )
    })
}

fn float_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    pair(
        opt(alt((tag("+"), tag("-")))),
        recognize(separated_pair(
            character::complete::i32,
            tag("."),
            character::complete::i32,
        )),
    )(source)
    .map(|(remain, (sign, value))| {
        let sign = sign
            .map(|value| String::from_utf8(value.to_vec()).unwrap())
            .unwrap_or_else(|| String::from("+"));
        let value = String::from_utf8(value.to_vec()).unwrap();

        (
            remain,
            arch::Argument::Float {
                value: f32::from_str(&format!("{sign}{value}")).unwrap(),
            },
        )
    })
}

fn string_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    delimited(tag("\""), take_till(|c| c == b'"'), tag("\""))(source).map(|(remain, value)| {
        (
            remain,
            arch::Argument::String {
                value: String::from_utf8(value.to_vec()).unwrap(),
            },
        )
    })
}

fn label_arg(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    alpha1(source).map(|(remain, value)| {
        (
            remain,
            arch::Argument::Label {
                name: String::from_utf8(value.to_vec()).unwrap(),
            },
        )
    })
}

fn arg_parser(source: &[u8]) -> IResult<&[u8], arch::Argument> {
    terminated(
        alt((
            self::register_arg,
            self::pin_arg,
            self::hex_int_arg,
            self::float_arg,
            self::int_arg,
            self::string_arg,
            self::label_arg,
        )),
        space0,
    )(source)
    .map(|(remain, arg)| (remain, arg))
}

fn operation_parser(source: &[u8]) -> IResult<&[u8], String> {
    terminated(alpha1, alt((line_ending, eof, space1)))(source).map(|(remain, name)| {
        (
            remain,
            String::from_utf8(name.to_vec()).unwrap().to_lowercase(),
        )
    })
}

#[cfg(test)]
mod tests {
    use arch::Token;

    #[test]
    fn parse_single_instruction() {
        let instruction = super::parse("add".as_bytes());

        assert_eq!(
            instruction,
            Ok((
                &[] as &[u8],
                Token::Instruction {
                    operation: arch::Operation::Add,
                    args: Vec::new()
                }
            ))
        )
    }

    #[test]
    fn parse_instruction_with_args() {
        let instruction = super::parse(
            "add r1 p4 78 -99 0xFF -0xDD 12.66 -4.12 \"Hello, world!\" start".as_bytes(),
        );

        assert_eq!(
            instruction,
            Ok((
                &[] as &[u8],
                Token::Instruction {
                    operation: arch::Operation::Add,
                    args: vec![
                        arch::Argument::Register { id: 1 },
                        arch::Argument::Pin { id: 4 },
                        arch::Argument::Int { value: 78 },
                        arch::Argument::Int { value: -99 },
                        arch::Argument::Int { value: 0xFF },
                        arch::Argument::Int { value: -0xDD },
                        arch::Argument::Float { value: 12.66 },
                        arch::Argument::Float { value: -4.12 },
                        arch::Argument::String {
                            value: String::from("Hello, world!")
                        },
                        arch::Argument::Label {
                            name: String::from("start")
                        }
                    ]
                }
            ))
        );
    }
}