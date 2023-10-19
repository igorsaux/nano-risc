use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, not_line_ending},
    combinator::eof,
    sequence::delimited,
    IResult,
};

use super::{ParsingError, Span, Token, TokenKind};

pub fn parse(data: Span) -> IResult<Span, Vec<Token>, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    delimited(tag("#"), not_line_ending, alt((eof, line_ending)))(data).map(|(remain, text)| {
        (
            remain,
            vec![Token {
                location,
                kind: TokenKind::Comment {
                    text: String::from_utf8(text.to_vec()).unwrap(),
                },
            }],
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::parser::{Span, Token, TokenKind};
    use nano_risc_arch::{Location, SourceUnit};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_comment() {
        let unit = SourceUnit::new_anonymous("# Test comment".as_bytes().to_vec());
        let comment = super::parse(Span::new_extra(unit.data(), unit.clone()));

        assert_eq!(
            comment.map(|(_, token)| token),
            Ok(vec![Token {
                location: Location {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                kind: TokenKind::Comment {
                    text: String::from(" Test comment")
                }
            }])
        )
    }
}
