use crate::parser::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxToken {
    pub token: Token,
    pub child: Vec<SyntaxToken>,
}
