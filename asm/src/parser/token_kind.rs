use super::ArgumentToken;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Comment { text: String },
    Label { name: String },
    Operation { operation: String },
    Argument { argument: ArgumentToken },
}
