use arch::Token;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::eof,
    sequence::delimited,
    IResult,
};

pub fn parse(input: &[u8]) -> IResult<&[u8], Token> {
    delimited(tag("#"), not_line_ending, alt((eof, line_ending)))(input).map(|(remain, text)| {
        (
            remain,
            Token::Comment {
                text: String::from_utf8(text.to_vec()).unwrap(),
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use arch::Token;

    #[test]
    fn parse_comment() {
        let comment = super::parse("# Test comment".as_bytes());

        assert_eq!(
            comment,
            Ok((
                &[] as &[u8],
                Token::Comment {
                    text: String::from(" Test comment")
                }
            ))
        )
    }
}
