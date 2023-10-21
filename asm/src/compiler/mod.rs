mod ast;
mod compilation_error;
mod compilation_error_kind;
mod syntax_token;

use crate::parser::{ArgumentToken, Token, TokenKind};
use nano_risc_arch::{
    Argument, Assembly, DebugInfo, Instruction, Limits, Location, Operation, SourceUnit,
};
use std::{collections::BTreeMap, str::FromStr};

pub use ast::Ast;
pub use compilation_error::CompilationError;
pub use compilation_error_kind::CompilationErrorKind;
pub use syntax_token::SyntaxToken;

pub fn compile(
    unit: SourceUnit,
    tokens: Vec<Token>,
    limits: &Limits,
) -> Result<Assembly, CompilationError> {
    let ast = Ast::new(&tokens)?;

    let mut string_positions = BTreeMap::<String, usize>::new();
    let mut source_loc = BTreeMap::<usize, Location>::new();
    let mut assembly = Assembly {
        instructions: Vec::new(),
        debug_info: None,
        code_section_size: nano_risc_arch::math::align_to_mult(ast.tokens.len(), 4),
        text_section: Vec::new(),
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
                            ArgumentToken::Int { value } => Argument::Int { value },
                            ArgumentToken::Float { value } => Argument::Float { value },
                            ArgumentToken::String { value } => {
                                if string_positions.contains_key(&value) {
                                    Argument::Int {
                                        value: string_positions[&value] as i32,
                                    }
                                } else {
                                    let position = (assembly.text_section.len()
                                        + assembly.code_section_size)
                                        as i32;

                                    if position == i32::MAX {
                                        return Err(CompilationError::new(
                                            format!(
                                                "Assembly' text section is too large: {position}"
                                            ),
                                            syntax.token.location,
                                            CompilationErrorKind::TooLargeAssembly {
                                                size: position as usize,
                                            },
                                        ));
                                    }

                                    string_positions.insert(value.clone(), position as usize);
                                    assembly.text_section.append(&mut value.into_bytes());
                                    assembly.text_section.push(0);

                                    Argument::Int { value: position }
                                }
                            }
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
                            ArgumentToken::Constant { name } => match name.as_str() {
                                "data" => Argument::Int {
                                    value: assembly.code_section_size as i32,
                                },
                                "ram_end" => Argument::Int {
                                    value: limits.ram_length as i32,
                                },
                                _ => {
                                    return Err(CompilationError::new(
                                        format!("Unknown constant: {name}"),
                                        syntax.token.location,
                                        CompilationErrorKind::UnknownConstant { name },
                                    ))
                                }
                            },
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

        let size = assembly.instructions.len();

        if size + assembly.text_section.len() >= limits.ram_length {
            return Err(CompilationError::new(
                format!("Assembly is too large to be fitted into RAM: {size}"),
                Location::default(),
                CompilationErrorKind::TooLargeAssembly { size },
            ));
        }
    }

    assembly.debug_info = Some(DebugInfo { source_loc, unit });

    Ok(assembly)
}

#[cfg(test)]
mod tests {
    use crate::{compiler, parser};
    use nano_risc_arch::{
        Argument, Assembly, DebugInfo, Instruction, Limits, Location, Operation, RegisterKind,
        RegisterMode, SourceUnit,
    };
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn compilation() {
        let source = r#"
            add $r0 1 0
            start:
            mov $r1 5
            jmp start
        "#;

        let unit = SourceUnit::new_anonymous(source.as_bytes().to_vec());
        let tokens = parser::parse(&unit).unwrap();
        let assembly = compiler::compile(unit, tokens, &Limits::default());
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
                                register: RegisterKind::Regular {
                                    id: 0,
                                    mode: RegisterMode::Direct
                                }
                            },
                            Argument::Int { value: 1 },
                            Argument::Int { value: 0 }
                        ]
                    },
                    Instruction {
                        operation: Operation::Mov,
                        arguments: vec![
                            Argument::Register {
                                register: RegisterKind::Regular {
                                    id: 1,
                                    mode: RegisterMode::Direct
                                }
                            },
                            Argument::Int { value: 5 },
                        ]
                    },
                    Instruction {
                        operation: Operation::Jmp,
                        arguments: vec![Argument::Int { value: 1 },]
                    }
                ],
                debug_info: Some(DebugInfo { source_loc, unit }),
                text_section: Vec::new(),
                code_section_size: 4
            })
        )
    }
}
