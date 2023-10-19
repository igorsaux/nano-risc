use arch::{Argument, Operation, RegisterKind, Token};

use crate::{Program, RuntimeError, VMStatus, Value, REGISTERS_COUNT};

pub type DbgCallback = Box<dyn Fn(String)>;

#[derive(Default)]
pub struct VM {
    registers: [Value; crate::REGISTERS_COUNT],
    stack: [Option<Value>; crate::STACK_SIZE],
    program: Option<Program>,
    pc: usize,
    sp: usize,
    dbg_callback: Option<DbgCallback>,
}

impl std::fmt::Debug for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VM")
            .field("registers", &self.registers)
            .field("stack", &self.stack)
            .field("program", &self.program)
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .finish()
    }
}

impl VM {
    pub fn load_program(&mut self, program: Program) {
        self.program = Some(program);
    }

    pub fn unload_program(&mut self) {
        self.program = None;
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = Default::default();
        self.stack = Default::default();
    }

    pub fn registers(&self) -> &[Value; crate::REGISTERS_COUNT] {
        &self.registers
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn set_dbg_callback(&mut self, callback: DbgCallback) {
        self.dbg_callback = Some(callback)
    }

    /// Executes 1 instruction.
    pub fn tick(&mut self) -> Result<VMStatus, RuntimeError> {
        let Some(program) = self.program.as_ref() else {
            return Ok(VMStatus::Idle);
        };

        match self.pc.cmp(&program.tokens.len()) {
            std::cmp::Ordering::Equal => return Ok(VMStatus::Finished),
            std::cmp::Ordering::Greater => {
                return Err(RuntimeError::InvalidPosition { position: self.pc })
            }
            _ => {}
        };

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

    pub fn write_register(
        &mut self,
        register: RegisterKind,
        value: Value,
    ) -> Result<(), RuntimeError> {
        match register {
            RegisterKind::Regular { id } => {
                if id >= REGISTERS_COUNT {
                    return Err(RuntimeError::InvalidRegister { register });
                }

                self.registers[id] = value;
            }
            RegisterKind::ProgramCounter => match value {
                Value::Float { value } => self.pc = value as usize,
                Value::String { .. } => {
                    return Err(RuntimeError::InvalidType {
                        message: String::from("PC does not accept string"),
                    })
                }
            },
            RegisterKind::StackPointer => {
                return Err(RuntimeError::RegisterIsReadOnly { register })
            }
        };

        Ok(())
    }

    fn execute_instruction(
        &mut self,
        operation: Operation,
        args: &[Argument],
    ) -> Result<Option<VMStatus>, RuntimeError> {
        match operation {
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div | Operation::Mod => {
                let Argument::Register { register } = args[0] else {
                    unreachable!()
                };

                let a = self.argument_to_float(&args[1])?;
                let b = self.argument_to_float(&args[2])?;
                let result = match operation {
                    Operation::Add => Value::Float { value: a + b },
                    Operation::Sub => Value::Float { value: a - b },
                    Operation::Mul => Value::Float { value: a * b },
                    Operation::Div => {
                        if b == 0.0 {
                            return Err(RuntimeError::DividedByZero);
                        }

                        Value::Float { value: a / b }
                    }
                    Operation::Mod => {
                        if b == 0.0 {
                            return Err(RuntimeError::DividedByZero);
                        }

                        Value::Float { value: a % b }
                    }
                    _ => unreachable!(),
                };

                self.write_register(register, result)?;
            }
            Operation::Mov => {
                let a = self.argument_to_value(&args[0])?;

                let Argument::Register { register } = args[1] else {
                    unreachable!()
                };

                self.write_register(register, a)?
            }
            Operation::Jmp => {
                let value = self.argument_to_value(&args[0])?;

                self.write_register(RegisterKind::ProgramCounter, value)?
            }
            Operation::Dbg => {
                let Some(callback) = &self.dbg_callback else {
                    return Ok(None);
                };

                let value = self.argument_to_value(&args[0])?;
                let text = match value {
                    Value::Float { value } => format!("{value}"),
                    Value::String { value } => value.to_string(),
                };

                callback(text)
            }
            Operation::Yield => return Ok(Some(VMStatus::Yield)),
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
                    self.write_register(
                        RegisterKind::ProgramCounter,
                        self.argument_to_value(&args[2])?,
                    )?
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
                    self.write_register(
                        RegisterKind::ProgramCounter,
                        self.argument_to_value(&args[1])?,
                    )?
                }
            }
            Operation::Seq
            | Operation::Sge
            | Operation::Sgt
            | Operation::Sle
            | Operation::Slt
            | Operation::Sne => {
                let Argument::Register { register } = &args[0] else {
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

                self.write_register(
                    *register,
                    Value::Float {
                        value: if result { 1.0 } else { 0.0 },
                    },
                )?
            }
            Operation::Seqz
            | Operation::Sgez
            | Operation::Sgtz
            | Operation::Slez
            | Operation::Sltz
            | Operation::Snez => {
                let Argument::Register { register } = &args[0] else {
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

                self.write_register(
                    *register,
                    Value::Float {
                        value: if result { 1.0 } else { 0.0 },
                    },
                )?
            }
            Operation::Halt => return Ok(Some(VMStatus::Finished)),
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
            Argument::Register { register: kind } => self.register_to_value(*kind),
            Argument::Int { value } => Ok(Value::Float {
                value: *value as f32,
            }),
            Argument::Float { value } => Ok(Value::Float { value: *value }),
            Argument::String { value } => Ok(Value::String {
                value: value.clone(),
            }),
            Argument::Label { name } => {
                let Some(program) = &self.program else {
                    unreachable!()
                };

                Ok(Value::Float {
                    value: program.labels[name] as f32,
                })
            }
            // Argument types should be checked at compilation time
            _ => unreachable!(),
        }
    }
}
