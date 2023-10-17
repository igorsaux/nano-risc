use super::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Assembly {
    pub tokens: Vec<Token>,
}
