use crate::{
    Argument, AssemblyError, AssemblyErrorKind, DebugInfo, Instruction, Limits, Location,
    Operation, RegisterKind,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Assembly {
    pub debug_info: Option<DebugInfo>,
    pub instructions: Vec<Instruction>,
    pub code_section_size: usize,
    pub text_section: Vec<u8>,
}

impl Assembly {
    pub fn validate(&self, limits: &Limits) -> Result<(), AssemblyError> {
        for (address, instruction) in self.instructions.iter().enumerate() {
            let dbg = self
                .debug_info
                .as_ref()
                .map(|debug_info| (address, debug_info));

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
                    register: RegisterKind::Regular { id, .. },
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
                        register: RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                            | RegisterKind::Pin { .. }
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
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
            }
            Operation::Dbgs => {
                if args.len() != 1 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 1 argument"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
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
                    Argument::Register {
                        register: RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                            | RegisterKind::Pin { .. }
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
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
                        register: RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                            | RegisterKind::Pin { .. }
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
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
                        register: RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                            | RegisterKind::Pin { .. }
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Push => {
                if args.len() != 1 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 1 argument"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                };
            }
            Operation::Pop | Operation::Peek => {
                if args.len() != 1 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 1 argument"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                };

                if !matches!(
                    args[0],
                    Argument::Register {
                        register: RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                            | RegisterKind::Pin { .. }
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Ret => {
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
            Operation::Call => {
                if args.len() != 1 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 1 argument"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::And
            | Operation::Or
            | Operation::Xor
            | Operation::Nor
            | Operation::Andi
            | Operation::Ori
            | Operation::Xori
            | Operation::Shr
            | Operation::Shl
            | Operation::Ror
            | Operation::Rol => {
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
                        register: RegisterKind::Pin { .. }
                            | RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Sqrt
            | Operation::Trunc
            | Operation::Ceil
            | Operation::Floor
            | Operation::Abs
            | Operation::Exp
            | Operation::Inf
            | Operation::Nan => {
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
                        register: RegisterKind::Pin { .. }
                            | RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Max | Operation::Min | Operation::Log => {
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
                        register: RegisterKind::Pin { .. }
                            | RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Lb | Operation::Lh | Operation::Lw => {
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
                        register: RegisterKind::Pin { .. }
                            | RegisterKind::Regular { .. }
                            | RegisterKind::ProgramCounter
                    }
                ) {
                    return Err(AssemblyError::new(
                        format!("{op}'s first argument accepts only registers"),
                        Self::get_loc(dbg),
                        AssemblyErrorKind::InvalidInstruction {
                            name: op.to_string(),
                        },
                    ));
                }
            }
            Operation::Sb | Operation::Sh | Operation::Sw => {
                if args.len() != 2 {
                    return Err(AssemblyError::new(
                        format!("{op} requires 2 arguments"),
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
