use crate::{
    Argument, AssemblyError, AssemblyErrorKind, DebugInfo, Instruction, Limits, Location,
    Operation, RegisterKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Assembly {
    pub instructions: Vec<Instruction>,
    pub debug_info: Option<DebugInfo>,
}

impl Assembly {
    pub fn validate(&self, limits: &Limits) -> Result<(), AssemblyError> {
        for (line, instruction) in self.instructions.iter().enumerate() {
            let dbg = self
                .debug_info
                .as_ref()
                .map(|debug_info| (line, debug_info));

            Self::validate_instruction(instruction, limits, dbg)?;
        }

        Ok(())
    }

    fn get_loc(dbg: Option<(usize, &DebugInfo)>) -> Option<Location> {
        if let Some((line, info)) = &dbg {
            Some(info.source_loc[&line])
        } else {
            None
        }
    }

    fn validate_arguments(
        instruction: &Instruction,
        limits: &Limits,
        dbg: Option<(usize, &DebugInfo)>,
    ) -> Result<(), AssemblyError> {
        for arg in &instruction.arguments {
            match arg {
                Argument::Register {
                    register: RegisterKind::Regular { id },
                } => {
                    if *id >= limits.regular_registers {
                        return Err(AssemblyError::new(
                            format!(
                                "Register's id {id} is out of bounds (maximum: {})",
                                limits.regular_registers - 1
                            ),
                            Self::get_loc(dbg),
                            AssemblyErrorKind::InvalidRegister { id: *id },
                        ));
                    }
                }
                Argument::Pin { id } => {
                    if *id >= limits.pins {
                        return Err(AssemblyError::new(
                            format!(
                                "Pin's id {id} is out of bounds (maximum: {})",
                                limits.pins - 1
                            ),
                            Self::get_loc(dbg),
                            AssemblyErrorKind::InvalidPin { id: *id },
                        ));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn validate_instruction(
        instruction: &Instruction,
        limits: &Limits,
        dbg: Option<(usize, &DebugInfo)>,
    ) -> Result<(), AssemblyError> {
        Self::validate_arguments(instruction, limits, dbg)?;

        let Instruction {
            arguments: args,
            operation: op,
        } = &instruction;

        match op {
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div | Operation::Mod => {
                if args.len() != 3 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 3 arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Register {
                        register: RegisterKind::Regular { .. } | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only regular registers and pc"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[1],
                    Argument::Int { .. } | Argument::Float { .. } | Argument::Register { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s second argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[2],
                    Argument::Int { .. } | Argument::Float { .. } | Argument::Register { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s third argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Jmp => {
                if args.is_empty()
                    || !matches!(args[0], Argument::Int { .. } | Argument::Register { .. })
                {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Dbg => {
                if args.len() != 1 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 1 argument"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Int { .. }
                        | Argument::Float { .. }
                        | Argument::Register { .. }
                        | Argument::String { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op} accepts only numbers, strings and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                return Ok(());
            }
            Operation::Yield | Operation::Halt => {
                if !args.is_empty() {
                    return Err(AssemblyError::new(
                        format!("{op} does not accept arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Mov => {
                if args.len() != 2 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 2 arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Int { .. }
                        | Argument::Float { .. }
                        | Argument::Register { .. }
                        | Argument::String { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts numbers, strings and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[1],
                    Argument::Register {
                        register: RegisterKind::Regular { .. } | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s second argument accepts only regular registers and pc"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Beq
            | Operation::Bge
            | Operation::Bgt
            | Operation::Ble
            | Operation::Blt
            | Operation::Bne => {
                if args.len() != 3 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 3 arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Int { .. }
                        | Argument::Float { .. }
                        | Argument::Register { .. }
                        | Argument::String { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!(
                            "{op}'s first argument accepts only numbers, strings and registers"
                        ),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[1],
                    Argument::Int { .. }
                        | Argument::Float { .. }
                        | Argument::Register { .. }
                        | Argument::String { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!(
                            "{op}'s second argument accepts only numbers, strings and registers"
                        ),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(args[2], Argument::Int { .. } | Argument::Register { .. }) {
                    return Err(AssemblyError::new(
                        format!("{op}'s third argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if (matches!(args[0], Argument::String { .. })
                    && !matches!(args[1], Argument::String { .. }))
                    || (!matches!(args[0], Argument::String { .. })
                        && matches!(args[1], Argument::String { .. }))
                {
                    return Err(AssemblyError::new(
                        format!("{op}'s first and second arguments should be of the same type"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Beqz
            | Operation::Bgez
            | Operation::Bgtz
            | Operation::Blez
            | Operation::Bltz
            | Operation::Bnez => {
                if args.len() != 2 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 2 arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Int { .. }
                        | Argument::Float { .. }
                        | Argument::Register { .. }
                        | Argument::String { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!(
                            "{op}'s first argument accepts only numbers, strings and registers"
                        ),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(args[1], Argument::Int { .. } | Argument::Register { .. }) {
                    return Err(AssemblyError::new(
                        format!("{op}'s second argument accepts numbers, labels and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Seq
            | Operation::Sge
            | Operation::Sgt
            | Operation::Sle
            | Operation::Slt
            | Operation::Sne => {
                if args.len() != 3 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 3 arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Register {
                        register: RegisterKind::Regular { .. } | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only regular registers and pc"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[1],
                    Argument::Int { .. } | Argument::Float { .. } | Argument::Register { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s second argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[2],
                    Argument::Int { .. } | Argument::Float { .. } | Argument::Register { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s third argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if (matches!(args[1], Argument::String { .. })
                    && !matches!(args[2], Argument::String { .. }))
                    || (!matches!(args[1], Argument::String { .. })
                        && matches!(args[2], Argument::String { .. }))
                {
                    return Err(AssemblyError::new(
                        format!("{op}'s second and third arguments should be of the same type"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Seqz
            | Operation::Sgez
            | Operation::Sgtz
            | Operation::Slez
            | Operation::Sltz
            | Operation::Snez => {
                if args.len() != 2 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 2 arguments"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[0],
                    Argument::Register {
                        register: RegisterKind::Regular { .. } | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only regular registers and pc"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }

                if !matches!(
                    args[1],
                    Argument::Int { .. } | Argument::Float { .. } | Argument::Register { .. }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s second argument accepts only numbers and registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
        }

        Ok(())
    }
}
