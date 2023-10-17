use arch::RegisterKind;
use nom::{
    branch::alt, bytes::complete::tag, character, combinator::recognize, sequence::preceded,
    IResult,
};

pub fn parse(source: &[u8]) -> IResult<&[u8], RegisterKind> {
    alt((regular_register, program_counter, stack_pointer))(source)
}

fn regular_register(source: &[u8]) -> IResult<&[u8], RegisterKind> {
    preceded(tag("$r"), character::complete::u8)(source)
        .map(|(remain, id)| (remain, RegisterKind::Regular { id: id as usize }))
}

fn program_counter(source: &[u8]) -> IResult<&[u8], RegisterKind> {
    recognize(tag("$pc"))(source).map(|(remain, _)| (remain, RegisterKind::ProgramCounter))
}

fn stack_pointer(source: &[u8]) -> IResult<&[u8], RegisterKind> {
    recognize(tag("$sp"))(source).map(|(remain, _)| (remain, RegisterKind::StackPointer))
}
