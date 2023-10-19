use nano_risc_arch::Location;

use super::token_kind::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
}
