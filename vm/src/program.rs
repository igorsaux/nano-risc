use std::collections::BTreeMap;

use arch::{Argument, Assembly, Operation, RegisterKind, Token};

use crate::CompileError;

#[derive(Debug, Clone, Default)]
pub struct Program {
    pub labels: BTreeMap<String, usize>,
    pub tokens: Vec<Token>,
}

impl Program {
    pub fn try_compile(assembly: Assembly) -> Result<Self, CompileError> {
        let mut program = Program::default();

        for token in assembly.tokens.into_iter() {
            match token {
                Token::Label { name } => {
                    if program.labels.contains_key(&name) {
                        return Err(CompileError::DuplicatedLabel { name });
                    }

                    program.labels.insert(name, program.tokens.len());
                }
                Token::Instruction { operation, args } => {
                    validate_instruction(operation, &args)?;

                    program.tokens.push(Token::Instruction { operation, args })
                }
                _ => {}
            }
        }

        // Validate labels

        for token in &program.tokens {
            let Token::Instruction {
                operation: Operation::Jmp,
                args,
            } = token
            else {
                continue;
            };

            let Argument::Label { name } = &args[0] else {
                panic!("First argument should be a label")
            };

            if !program.labels.contains_key(name) {
                return Err(CompileError::DuplicatedLabel { name: name.clone() });
            }
        }

        Ok(program)
    }
}

fn validate_instruction(operation: Operation, args: &[Argument]) -> Result<(), CompileError> {
    for arg in args {
        default_argument_validation(arg)?;
    }

    match operation {
        Operation::Add | Operation::Sub | Operation::Mul | Operation::Div | Operation::Mod => {
            if args.len() != 3 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 3 arguments"),
                });
            }

            if !matches!(
                args[0],
                Argument::Register {
                    kind: RegisterKind::Regular { .. }
                }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s first argument accepts only registers"),
                });
            }

            if !matches!(
                args[1],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s second argument accepts only numbers and registers"
                    ),
                });
            }

            if !matches!(
                args[2],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s third argument accepts only numbers and registers"
                    ),
                });
            }
        }
        Operation::Jmp => {
            if args.is_empty() || !matches!(args[0], Argument::Label { .. }) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s first argument requires label argument"),
                });
            }
        }
        Operation::Dbg => {
            if args.len() != 1 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 1 argument"),
                });
            }

            if !matches!(
                args[0],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register { .. }
                    | Argument::String { .. }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} accepts only numbers, strings and registers"),
                });
            }

            return Ok(());
        }
        Operation::Yield => {
            if !args.is_empty() {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} does not accept arguments"),
                });
            }
        }
        Operation::Mov => {
            if args.len() != 2 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 2 arguments"),
                });
            }

            if !matches!(
                args[0],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
                    | Argument::String { .. }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s first argument accepts numbers, strings and registers"
                    ),
                });
            }

            if !matches!(
                args[1],
                Argument::Register {
                    kind: RegisterKind::Regular { .. }
                }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s second argument accepts only registers"),
                });
            }
        }
        Operation::Beq
        | Operation::Bge
        | Operation::Bgt
        | Operation::Ble
        | Operation::Blt
        | Operation::Bne => {
            if args.len() != 3 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 3 arguments"),
                });
            }

            if !matches!(
                args[0],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
                    | Argument::String { .. }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s first argument accepts only numbers, strings and registers"
                    ),
                });
            }

            if !matches!(
                args[1],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
                    | Argument::String { .. }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s second argument accepts only numbers, strings and registers"
                    ),
                });
            }

            if !matches!(args[2], Argument::Label { .. }) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s third argument accepts only labels"),
                });
            }

            if (matches!(args[0], Argument::String { .. })
                && !matches!(args[1], Argument::String { .. }))
                || (!matches!(args[0], Argument::String { .. })
                    && matches!(args[1], Argument::String { .. }))
            {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s first and second arguments should be of the same type"
                    ),
                });
            }
        }
        Operation::Beqz
        | Operation::Bgez
        | Operation::Bgtz
        | Operation::Blez
        | Operation::Bltz
        | Operation::Bnez => {
            if args.len() != 2 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 2 arguments"),
                });
            }

            if !matches!(
                args[0],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
                    | Argument::String { .. }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s first argument accepts only numbers, strings and registers"
                    ),
                });
            }

            if !matches!(args[1], Argument::Label { .. }) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s second argument accepts only labels"),
                });
            }
        }
        Operation::Seq
        | Operation::Sge
        | Operation::Sgt
        | Operation::Sle
        | Operation::Slt
        | Operation::Sne => {
            if args.len() != 3 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 3 arguments"),
                });
            }

            if !matches!(
                args[0],
                Argument::Register {
                    kind: RegisterKind::Regular { .. }
                }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s first argument accepts only registers"),
                });
            }

            if !matches!(
                args[1],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s second argument accepts only numbers and registers"
                    ),
                });
            }

            if !matches!(
                args[2],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s third argument accepts only numbers and registers"
                    ),
                });
            }

            if (matches!(args[1], Argument::String { .. })
                && !matches!(args[2], Argument::String { .. }))
                || (!matches!(args[1], Argument::String { .. })
                    && matches!(args[2], Argument::String { .. }))
            {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s second and third arguments should be of the same type"
                    ),
                });
            }
        }
        Operation::Seqz
        | Operation::Sgez
        | Operation::Sgtz
        | Operation::Slez
        | Operation::Sltz
        | Operation::Snez => {
            if args.len() != 2 {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?} requires 2 arguments"),
                });
            }

            if !matches!(
                args[0],
                Argument::Register {
                    kind: RegisterKind::Regular { .. }
                }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!("{operation:?}'s first argument accepts only registers"),
                });
            }

            if !matches!(
                args[1],
                Argument::Int { .. }
                    | Argument::Float { .. }
                    | Argument::Register {
                        kind: RegisterKind::Regular { .. }
                    }
            ) {
                return Err(CompileError::InvalidInstruction {
                    operation,
                    message: format!(
                        "{operation:?}'s second argument accepts only numbers and registers"
                    ),
                });
            }
        }
    }

    Ok(())
}

fn default_argument_validation(argument: &Argument) -> Result<(), CompileError> {
    match argument {
        Argument::Register {
            kind: RegisterKind::Regular { id },
        } => {
            if *id >= crate::REGISTERS_COUNT {
                return Err(CompileError::InvalidRegister { id: *id });
            }
        }
        Argument::Pin { id } => {
            if *id >= crate::MAX_PINS {
                return Err(CompileError::InvalidPin { id: *id });
            }
        }
        _ => return Ok(()),
    }

    Ok(())
}
