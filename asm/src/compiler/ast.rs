use std::collections::BTreeMap;

use crate::parser::{Token, TokenKind};

use super::{syntax_token::SyntaxToken, CompilationError, CompilationErrorKind};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ast {
    pub labels: BTreeMap<String, usize>,
    pub tokens: Vec<SyntaxToken>,
}

impl Ast {
    pub fn new(tokens: &[Token]) -> Result<Self, CompilationError> {
        let mut ast = Self::default();
        let mut current_operation: Option<SyntaxToken> = None;

        for token in tokens {
            match &token.kind {
                TokenKind::Comment { .. } | TokenKind::Label { .. } => {
                    if let Some(operation) = current_operation.as_ref() {
                        ast.tokens.push(operation.clone())
                    }

                    current_operation = None;

                    if let TokenKind::Label { name } = &token.kind {
                        if ast.labels.contains_key(name) {
                            return Err(CompilationError::new(
                                format!("Duplicate label: {name}"),
                                token.location,
                                CompilationErrorKind::DuplicateLabel { name: name.clone() },
                            ));
                        }

                        ast.labels.insert(name.clone(), ast.tokens.len());
                    }

                    continue;
                }
                TokenKind::Operation { .. } => {
                    if let Some(operation) = current_operation.as_ref() {
                        ast.tokens.push(operation.clone())
                    }

                    current_operation = Some(SyntaxToken {
                        token: token.clone(),
                        child: vec![],
                    });
                }
                TokenKind::Argument { .. } => {
                    let Some(operation) = current_operation.as_mut() else {
                        return Err(CompilationError::new(
                            String::from("Expected an operation but got an argument"),
                            token.location,
                            CompilationErrorKind::InvalidArgument,
                        ));
                    };

                    operation.child.push(SyntaxToken {
                        token: token.clone(),
                        child: Vec::new(),
                    });
                }
            }
        }

        if let Some(operation) = current_operation.as_ref() {
            ast.tokens.push(operation.clone())
        }

        Ok(ast)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        compiler::SyntaxToken,
        parser::{self, ArgumentToken, Token, TokenKind},
    };

    use super::Ast;
    use nano_risc_arch::{Location, RegisterKind, SourceUnit};
    use pretty_assertions::assert_eq;

    #[test]
    fn ast() {
        let source = r#"
start:
add $r0 1 0
"#;

        let tokens = parser::parse(&SourceUnit::new_anonymous(source.as_bytes().to_vec())).unwrap();
        let mut labels = BTreeMap::new();

        labels.insert(String::from("start"), 0);

        let ast = Ast::new(&tokens);

        assert_eq!(
            ast,
            Ok(Ast {
                labels,
                tokens: vec![SyntaxToken {
                    token: Token {
                        location: Location {
                            line: 3,
                            column: 1,
                            offset: 8
                        },
                        kind: TokenKind::Operation {
                            operation: String::from("add")
                        }
                    },
                    child: vec![
                        SyntaxToken {
                            token: Token {
                                location: Location {
                                    line: 3,
                                    column: 5,
                                    offset: 12
                                },
                                kind: TokenKind::Argument {
                                    argument: ArgumentToken::Register {
                                        register: RegisterKind::Regular { id: 0 }
                                    }
                                }
                            },
                            child: Vec::new()
                        },
                        SyntaxToken {
                            token: Token {
                                location: Location {
                                    line: 3,
                                    column: 9,
                                    offset: 16
                                },
                                kind: TokenKind::Argument {
                                    argument: ArgumentToken::Int { value: 1 }
                                }
                            },
                            child: Vec::new()
                        },
                        SyntaxToken {
                            token: Token {
                                location: Location {
                                    line: 3,
                                    column: 11,
                                    offset: 18
                                },
                                kind: TokenKind::Argument {
                                    argument: ArgumentToken::Int { value: 0 }
                                }
                            },
                            child: Vec::new()
                        }
                    ]
                }]
            })
        )
    }
}
