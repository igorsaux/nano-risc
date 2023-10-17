use arch::Token;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    combinator::eof,
    sequence::{pair, terminated},
    IResult,
};

pub fn parse(source: &[u8]) -> IResult<&[u8], Token> {
    terminated(pair(alpha1, tag(":")), alt((line_ending, eof, space1)))(source).map(
        |(remain, (name, _))| {
            (
                remain,
                Token::Label {
                    name: String::from_utf8(name.to_vec()).unwrap(),
                },
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use arch::Token;

    #[test]
    fn parse_label() {
        let label = super::parse("test:".as_bytes());

        assert_eq!(
            label,
            Ok((
                &[] as &[u8],
                Token::Label {
                    name: String::from("test")
                }
            ))
        )
    }
}
