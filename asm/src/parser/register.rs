use nano_risc_arch::{RegisterKind, RegisterMode};
use nom::{
    branch::alt, bytes::complete::tag, character, combinator::recognize, sequence::preceded,
    IResult,
};

use super::{ParsingError, ParsingErrorKind, Span};

pub fn parse(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    alt((regular_register, program_counter, stack_pointer))(data).map_err(
        |err: nom::Err<ParsingError>| {
            ParsingError::from_nom_error(
                String::from("Expected a regular register, pc or sp"),
                err,
                ParsingErrorKind::InvalidRegister,
            )
        },
    )
}

fn regular_register(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    alt((
        regular_indirect_register,
        regular_address_register,
        regular_direct_register,
    ))(data)
}

fn regular_indirect_register(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    preceded(tag("%r"), character::complete::u8)(data).map(|(remain, id)| {
        (
            remain,
            RegisterKind::Regular {
                id: id as usize,
                mode: RegisterMode::Indirect,
            },
        )
    })
}

fn regular_address_register(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    preceded(tag("@r"), character::complete::u8)(data).map(|(remain, id)| {
        (
            remain,
            RegisterKind::Regular {
                id: id as usize,
                mode: RegisterMode::Address,
            },
        )
    })
}

fn regular_direct_register(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    preceded(tag("$r"), character::complete::u8)(data).map(|(remain, id)| {
        (
            remain,
            RegisterKind::Regular {
                id: id as usize,
                mode: RegisterMode::Direct,
            },
        )
    })
}

fn program_counter(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    recognize(tag("$pc"))(data).map(|(remain, _)| (remain, RegisterKind::ProgramCounter))
}

fn stack_pointer(data: Span) -> IResult<Span, RegisterKind, ParsingError> {
    recognize(tag("$sp"))(data).map(|(remain, _)| (remain, RegisterKind::StackPointer))
}
