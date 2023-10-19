use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::eof,
    sequence::terminated,
    IResult,
};

use super::{ParsingError, ParsingErrorKind, Span, Token, TokenKind};

pub fn parse(data: Span) -> IResult<Span, Vec<Token>, ParsingError> {
    let location = data.extra.find_location(data.location_offset()).unwrap();

    terminated(
        terminated(alpha1, tag(":")),
        alt((line_ending, eof, space1)),
    )(data)
    .map(|(remain, name)| {
        (
            remain,
            vec![Token {
                location,
                kind: TokenKind::Label {
                    name: String::from_utf8(name.fragment().to_vec()).unwrap(),
                },
            }],
        )
    })
    .map_err(|err: nom::Err<ParsingError>| {
        ParsingError::from_nom_error(
            String::from("Invalid label"),
            err,
            ParsingErrorKind::InvalidLabel,
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::parser::{Span, Token, TokenKind};
    use nano_risc_arch::{Location, SourceUnit};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_label() {
        let unit = SourceUnit::new_anonymous("test:".as_bytes().to_vec());
        let label = super::parse(Span::new_extra(unit.data(), unit.clone()));

        assert_eq!(
            label.map(|(_, token)| token),
            Ok(vec![Token {
                location: Location {
                    line: 1,
                    column: 1,
                    offset: 0
                },
                kind: TokenKind::Label {
                    name: String::from("test")
                }
            }])
        )
    }
}
