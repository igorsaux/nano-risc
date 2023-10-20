mod ast;
mod compilation_error;
mod compilation_error_kind;
mod syntax_token;

use crate::parser::{ArgumentToken, Token, TokenKind};
use nano_risc_arch::{
    Argument, Assembly, DebugInfo, Instruction, Location, Operation, RegisterKind, SourceUnit,
};
use std::{collections::BTreeMap, str::FromStr};

pub use ast::Ast;
pub use compilation_error::CompilationError;
pub use compilation_error_kind::CompilationErrorKind;
pub use syntax_token::SyntaxToken;

pub fn compile(unit: SourceUnit, tokens: Vec<Token>) -> Result<Assembly, CompilationError> {
    let mut ast = Ast::new(&tokens)?;

    macros_stage(&mut ast)?;

    let mut source_loc: BTreeMap<usize, Location> = BTreeMap::new();
    let mut assembly = Assembly {
        instructions: Vec::new(),
        debug_info: None,
    };

    for (address, syntax) in ast.tokens.into_iter().enumerate() {
        let instruction = match syntax.token.kind {
            TokenKind::Operation { operation } => {
                let operation = Operation::from_str(&operation).map_err(|_| {
                    CompilationError::new(
                        format!("Invalid operation: {operation}"),
                        syntax.token.location,
                        CompilationErrorKind::InvalidOperation,
                    )
                })?;
                let arguments: Result<Vec<Argument>, CompilationError> = syntax
                    .child
                    .into_iter()
                    .map(|arg| {
                        let TokenKind::Argument { argument: arg } = arg.token.kind else {
                            panic!("Arguments should be after an operation")
                        };

                        let arg = match arg {
                            ArgumentToken::Register { register } => Argument::Register { register },
                            ArgumentToken::Pin { id } => Argument::Pin { id },
                            ArgumentToken::Int { value } => Argument::Int { value },
                            ArgumentToken::Float { value } => Argument::Float { value },
                            ArgumentToken::String { value } => Argument::String { value },
                            ArgumentToken::Label { name } => {
                                if !ast.labels.contains_key(&name) {
                                    return Err(CompilationError::new(
                                        format!("Unknown label: {name}"),
                                        syntax.token.location,
                                        CompilationErrorKind::UnknownLabel { name },
                                    ));
                                }

                                Argument::Int {
                                    value: ast.labels[&name] as i32,
                                }
                            }
                        };

                        Ok(arg)
                    })
                    .collect();

                Instruction {
                    operation,
                    arguments: arguments?,
                }
            }
            _ => panic!("Only operations should be on top level"),
        };

        source_loc.insert(address, syntax.token.location);
        assembly.instructions.push(instruction);
    }

    assembly.debug_info = Some(DebugInfo { source_loc, unit });

    Ok(assembly)
}

fn macros_stage(ast: &mut Ast) -> Result<(), CompilationError> {
    for token in &mut ast.tokens {
        match &token.token.kind {
            TokenKind::Operation { operation } => {
                if operation == "jmp" {
                    let old_tocken = token.token.clone();

                    token.token = Token {
                        kind: TokenKind::Operation {
                            operation: String::from("mov"),
                        },
                        location: old_tocken.location,
                    };

                    token.child.insert(
                        1,
                        SyntaxToken {
                            token: Token {
                                kind: TokenKind::Argument {
                                    argument: ArgumentToken::Register {
                                        register: RegisterKind::ProgramCounter,
                                    },
                                },
                                location: Location::default(),
                            },
                            child: Vec::new(),
                        },
                    )
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compiler, parser};
    use nano_risc_arch::{
        Argument, Assembly, DebugInfo, Instruction, Location, Operation, RegisterKind, SourceUnit,
    };
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn compilation() {
        let source = r#"
            add $r0 1 0
            start:
            mov 5 $r1
            jmp start
        "#;

        let unit = SourceUnit::new_anonymous(source.as_bytes().to_vec());
        let tokens = parser::parse(&unit).unwrap();
        let assembly = compiler::compile(unit, tokens);
        let mut source_loc = BTreeMap::new();
        let unit = SourceUnit::new_anonymous(source.as_bytes().to_vec());

        source_loc.insert(0, Location::new(2, 13, 13));
        source_loc.insert(1, Location::new(4, 13, 56));
        source_loc.insert(2, Location::new(5, 13, 78));

        assert_eq!(
            assembly,
            Ok(Assembly {
                instructions: vec![
                    Instruction {
                        operation: Operation::Add,
                        arguments: vec![
                            Argument::Register {
                                register: RegisterKind::Regular { id: 0 }
                            },
                            Argument::Int { value: 1 },
                            Argument::Int { value: 0 }
                        ]
                    },
                    Instruction {
                        operation: Operation::Mov,
                        arguments: vec![
                            Argument::Int { value: 5 },
                            Argument::Register {
                                register: RegisterKind::Regular { id: 1 }
                            }
                        ]
                    },
                    Instruction {
                        operation: Operation::Mov,
                        arguments: vec![
                            Argument::Register {
                                register: RegisterKind::ProgramCounter
                            },
                            Argument::Int { value: 1 }
                        ]
                    }
                ],
                debug_info: Some(DebugInfo { source_loc, unit })
            })
        )
    }
}
