use arch::{Argument, Operation, RegisterKind, Token};

use crate::{Program, RuntimeError, VMStatus, Value};

#[derive(Debug, Clone, Default)]
pub struct VM {
    pub registers: [Value; crate::REGISTERS_COUNT],
    pub stack: [Option<Value>; crate::STACK_SIZE],
    pub program: Option<Program>,
    pub pc: usize,
    pub sp: usize,
}

impl VM {
    pub fn load_program(&mut self, program: Program) {
        self.reset();
        self.program = Some(program);
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = Default::default();
        self.stack = Default::default();
        self.program = None;
    }

    /// Executes 1 instruction.
    pub fn tick(&mut self) -> Result<VMStatus, RuntimeError> {
        let Some(program) = self.program.as_ref() else {
            return Ok(VMStatus::Idle);
        };

        if self.pc >= program.tokens.len() {
            return Ok(VMStatus::Finished);
        }

        let token = &program.tokens[self.pc];

        self.pc += 1;

        if let Token::Instruction { operation, args } = token {
            // bruh
            let result = Self::execute_instruction(
                unsafe { &mut *(self as *const VM as *mut VM) },
                *operation,
                args,
            )?;

            if let Some(status) = result {
                return Ok(status);
            }
        }

        Ok(VMStatus::Running)
    }

    fn execute_instruction(
        &mut self,
        operation: Operation,
        args: &[Argument],
    ) -> Result<Option<VMStatus>, RuntimeError> {
        match operation {
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div => {
                let Argument::Register {
                    kind: RegisterKind::Regular { id: register },
                } = args[0]
                else {
                    unreachable!()
                };

                let a = self.argument_to_float(&args[1])?;
                let b = self.argument_to_float(&args[2])?;

                match operation {
                    Operation::Add => self.registers[register] = Value::Float { value: a + b },
                    Operation::Sub => self.registers[register] = Value::Float { value: a - b },
                    Operation::Mul => self.registers[register] = Value::Float { value: a * b },
                    Operation::Div => {
                        if b == 0.0 {
                            return Err(RuntimeError::DividedByZero);
                        }

                        self.registers[register] = Value::Float { value: a / b }
                    }
                    _ => unreachable!(),
                }
            }
            Operation::Mov => {
                let a = self.argument_to_value(&args[0])?;

                let Argument::Register {
                    kind: RegisterKind::Regular { id: register },
                } = args[1]
                else {
                    unreachable!()
                };

                self.registers[register] = a;
            }
            Operation::Jmp => {
                let Some(program) = &self.program else {
                    unreachable!()
                };

                let Argument::Label { name } = &args[0] else {
                    unreachable!()
                };

                self.pc = program.labels[name];
            }
            Operation::Dbg => {
                let value = self.argument_to_value(&args[0])?;

                match value {
                    Value::Float { value } => println!("{value}"),
                    Value::String { value } => println!("{value}"),
                }
            }
            Operation::Yield => return Ok(Some(VMStatus::Yield)),
            Operation::Mod => {
                let Argument::Register {
                    kind: RegisterKind::Regular { id: register },
                } = args[0]
                else {
                    unreachable!()
                };

                let a = self.argument_to_float(&args[1])?;
                let b = self.argument_to_float(&args[2])?;

                if b == 0.0 {
                    return Err(RuntimeError::DividedByZero);
                }

                self.registers[register] = Value::Float { value: a % b };
            }
            Operation::Beq
            | Operation::Bge
            | Operation::Bgt
            | Operation::Ble
            | Operation::Blt
            | Operation::Bne => {
                let a = self.argument_to_value(&args[0])?;
                let b = self.argument_to_value(&args[1])?;

                let result = match operation {
                    Operation::Beq => a == b,
                    Operation::Bge => a >= b,
                    Operation::Bgt => a > b,
                    Operation::Ble => a <= b,
                    Operation::Blt => a < b,
                    Operation::Bne => a != b,
                    _ => unreachable!(),
                };

                if result {
                    let Some(program) = &self.program else {
                        unreachable!()
                    };

                    let Argument::Label { name } = &args[2] else {
                        unreachable!()
                    };

                    self.pc = program.labels[name]
                }
            }
            Operation::Beqz
            | Operation::Bgez
            | Operation::Bgtz
            | Operation::Blez
            | Operation::Bltz
            | Operation::Bnez => {
                let a = self.argument_to_float(&args[0])?;

                let result = match operation {
                    Operation::Beqz => a == 0.0,
                    Operation::Bgez => a >= 0.0,
                    Operation::Bgtz => a > 0.0,
                    Operation::Blez => a <= 0.0,
                    Operation::Bltz => a < 0.0,
                    Operation::Bnez => a != 0.0,
                    _ => unreachable!(),
                };

                if result {
                    let Some(program) = &self.program else {
                        unreachable!()
                    };

                    let Argument::Label { name } = &args[1] else {
                        unreachable!()
                    };

                    self.pc = program.labels[name]
                }
            }
            Operation::Seq
            | Operation::Sge
            | Operation::Sgt
            | Operation::Sle
            | Operation::Slt
            | Operation::Sne => {
                let Argument::Register {
                    kind: RegisterKind::Regular { id: register },
                } = &args[0]
                else {
                    unreachable!()
                };
                let a = self.argument_to_value(&args[1])?;
                let b = self.argument_to_value(&args[2])?;

                let result = match operation {
                    Operation::Seq => a == b,
                    Operation::Sge => a >= b,
                    Operation::Sgt => a > b,
                    Operation::Sle => a <= b,
                    Operation::Slt => a < b,
                    Operation::Sne => a != b,
                    _ => unreachable!(),
                };

                self.registers[*register] = Value::Float {
                    value: if result { 1.0 } else { 0.0 },
                }
            }
            Operation::Seqz
            | Operation::Sgez
            | Operation::Sgtz
            | Operation::Slez
            | Operation::Sltz
            | Operation::Snez => {
                let Argument::Register {
                    kind: RegisterKind::Regular { id: register },
                } = &args[0]
                else {
                    unreachable!()
                };
                let a = self.argument_to_float(&args[1])?;

                let result = match operation {
                    Operation::Seqz => a == 0.0,
                    Operation::Sgez => a >= 0.0,
                    Operation::Sgtz => a > 0.0,
                    Operation::Slez => a <= 0.0,
                    Operation::Sltz => a < 0.0,
                    Operation::Snez => a != 0.0,
                    _ => unreachable!(),
                };

                self.registers[*register] = Value::Float {
                    value: if result { 1.0 } else { 0.0 },
                }
            }
        }

        Ok(None)
    }

    fn register_to_value(&self, register: RegisterKind) -> Result<Value, RuntimeError> {
        match register {
            RegisterKind::Regular { id } => Ok(self.registers[id].clone()),
            RegisterKind::ProgramCounter => Ok(Value::Float {
                value: self.pc as f32,
            }),
            RegisterKind::StackPointer => Ok(Value::Float {
                value: self.sp as f32,
            }),
        }
    }

    fn argument_to_float(&self, argument: &Argument) -> Result<f32, RuntimeError> {
        match self.argument_to_value(argument)? {
            Value::Float { value } => Ok(value),
            Value::String { .. } => Err(RuntimeError::InvalidType {
                message: String::from("Got string, but number required"),
            }),
        }
    }

    fn argument_to_value(&self, argument: &Argument) -> Result<Value, RuntimeError> {
        match argument {
            Argument::Register { kind } => self.register_to_value(*kind),
            Argument::Int { value } => Ok(Value::Float {
                value: *value as f32,
            }),
            Argument::Float { value } => Ok(Value::Float { value: *value }),
            Argument::String { value } => Ok(Value::String {
                value: value.clone(),
            }),
            // Argument types should be checked at compilation time
            _ => unreachable!("Invalid argument"),
        }
    }
}
