use crate::{RuntimeError, RuntimeErrorKind, VMStatus};
use nano_risc_arch::{
    Argument, Assembly, AssemblyError, Instruction, Limits, Operation, RegisterKind, RegisterMode,
};
use std::{cmp::Ordering, fmt::Debug};

pub type DbgCallback = Box<dyn Fn(String)>;

pub struct VM {
    limits: Limits,
    registers: Vec<f32>,
    stack: Vec<f32>,
    assembly: Option<Assembly>,
    pc: usize,
    sp: usize,
    dbg_callback: Option<DbgCallback>,
    status: VMStatus,
}

impl Default for VM {
    fn default() -> Self {
        Self::new(Limits::default())
    }
}

impl Debug for VM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VM")
            .field("limits", &self.limits)
            .field("registers", &self.registers)
            .field("stack", &self.stack)
            .field("assembly", &self.assembly)
            .field("pc", &self.pc)
            .field("sp", &self.sp)
            .field("status", &self.status)
            .finish()
    }
}

impl VM {
    pub fn new(limits: Limits) -> Self {
        let mut registers = Vec::with_capacity(limits.regular_registers);

        {
            let remains = registers.spare_capacity_mut();

            for value in remains {
                value.write(0.0);
            }

            unsafe { registers.set_len(limits.regular_registers) };
        }

        let mut stack = Vec::with_capacity(limits.stack_size);

        {
            let remains = stack.spare_capacity_mut();

            for value in remains {
                value.write(0.0);
            }

            unsafe { stack.set_len(limits.stack_size) };
        }

        Self {
            limits,
            registers,
            stack,
            assembly: None,
            pc: 0,
            sp: 0,
            dbg_callback: None,
            status: VMStatus::Idle,
        }
    }

    pub fn load_assembly(&mut self, assembly: Assembly) -> Result<(), AssemblyError> {
        assembly.validate(&self.limits)?;

        self.assembly = Some(assembly);

        Ok(())
    }

    pub fn unload_assembly(&mut self) {
        self.assembly = None;
    }

    pub fn assembly(&self) -> Option<&Assembly> {
        self.assembly.as_ref()
    }

    pub fn reset(&mut self) {
        self.status = VMStatus::Idle;
        self.pc = 0;
        self.sp = 0;

        for register in &mut self.registers {
            *register = 0.0;
        }

        for stack in &mut self.stack {
            *stack = 0.0;
        }
    }

    pub fn registers(&self) -> &[f32] {
        &self.registers
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn sp(&self) -> usize {
        self.sp
    }

    pub fn stack(&self) -> &[f32] {
        &self.stack
    }

    pub fn status(&self) -> VMStatus {
        self.status
    }

    pub fn set_dbg_callback(&mut self, callback: DbgCallback) {
        self.dbg_callback = Some(callback)
    }

    /// Executes 1 instruction.
    pub fn tick(&mut self) -> Result<VMStatus, RuntimeError> {
        match self.status {
            VMStatus::Finished | VMStatus::Error => return Ok(self.status),
            _ => {}
        }

        let Some(program) = self.assembly.as_ref() else {
            self.status = VMStatus::Idle;

            return Ok(self.status);
        };

        match self.pc.cmp(&program.instructions.len()) {
            Ordering::Equal => {
                self.status = VMStatus::Finished;

                return Ok(self.status);
            }
            Ordering::Greater => {
                return Err(RuntimeError::new(
                    format!(
                        "Address {} is out of bounds ({})",
                        self.pc,
                        program.instructions.len() - 1
                    ),
                    RuntimeErrorKind::InvalidAddress { address: self.pc },
                ));
            }
            _ => {}
        };

        let old_pc = self.pc;
        let Instruction {
            operation: op,
            arguments: args,
        } = &program.instructions[old_pc];

        // bruh
        let status =
            Self::execute_instruction(unsafe { &mut *(self as *const VM as *mut VM) }, *op, args)?;

        if let Some(status) = status {
            self.status = status;
        } else {
            self.status = VMStatus::Running;
        }

        if matches!(self.status, VMStatus::Running | VMStatus::Yield) && old_pc == self.pc() {
            self.write_register(RegisterKind::ProgramCounter, (old_pc + 1) as f32)?;
        }

        Ok(self.status)
    }

    pub fn write_register(
        &mut self,
        register: RegisterKind,
        value: f32,
    ) -> Result<(), RuntimeError> {
        match register {
            RegisterKind::Regular { id, mode } => {
                if id >= self.limits.regular_registers {
                    return Err(RuntimeError::new(
                        format!("Register {register} is out of maximum registers"),
                        RuntimeErrorKind::InvalidRegister { register },
                    ));
                }

                match mode {
                    RegisterMode::Direct => self.registers[id] = value,
                    RegisterMode::Indirect => {
                        self.write_register(
                            RegisterKind::Regular {
                                id: self.registers[id] as usize,
                                mode: RegisterMode::Direct,
                            },
                            value,
                        )?;
                    }
                    RegisterMode::Address => todo!(),
                }
            }
            RegisterKind::ProgramCounter => self.pc = value as usize,
            RegisterKind::StackPointer => {
                return Err(RuntimeError::new(
                    String::from("sp is read-only"),
                    RuntimeErrorKind::RegisterIsReadOnly { register },
                ))
            }
            RegisterKind::Pin { .. } => todo!(),
        };

        Ok(())
    }

    pub fn push_stack(&mut self, value: f32) -> Result<(), RuntimeError> {
        if self.sp >= self.stack.len() {
            return Err(RuntimeError::new(
                String::from("Stack overflow"),
                RuntimeErrorKind::StackOverflow,
            ));
        }

        self.stack[self.sp] = value;
        self.sp += 1;

        Ok(())
    }

    pub fn pop_stack(&mut self) -> Result<f32, RuntimeError> {
        if self.sp == 0 {
            return Err(RuntimeError::new(
                String::from("Stack overflow"),
                RuntimeErrorKind::StackOverflow,
            ));
        }

        self.sp -= 1;
        let a = self.stack[self.sp];
        self.stack[self.sp] = 0.0;

        Ok(a)
    }

    pub fn peek_stack(&mut self) -> Result<f32, RuntimeError> {
        if self.sp == 0 {
            return Err(RuntimeError::new(
                String::from("Stack overflow"),
                RuntimeErrorKind::StackOverflow,
            ));
        }

        Ok(self.stack[self.sp - 1])
    }

    fn execute_instruction(
        &mut self,
        operation: Operation,
        args: &[Argument],
    ) -> Result<Option<VMStatus>, RuntimeError> {
        match operation {
            Operation::Add | Operation::Sub | Operation::Mul | Operation::Div | Operation::Mod => {
                let Argument::Register { register } = args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };

                let a = self.argument_to_float(&args[1])?;
                let b = self.argument_to_float(&args[2])?;
                let result = match operation {
                    Operation::Add => a + b,
                    Operation::Sub => a - b,
                    Operation::Mul => a * b,
                    Operation::Div => {
                        if b == 0.0 {
                            return Err(RuntimeError::new(
                                String::from("Divide by zero"),
                                RuntimeErrorKind::DividedByZero,
                            ));
                        }

                        a / b
                    }
                    Operation::Mod => {
                        if b == 0.0 {
                            return Err(RuntimeError::new(
                                String::from("Divide by zero"),
                                RuntimeErrorKind::DividedByZero,
                            ));
                        }

                        a % b
                    }
                    _ => unreachable!(),
                };

                self.write_register(register, result)?;
            }
            Operation::Mov => {
                let Argument::Register { register } = args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };

                let a = self.argument_to_float(&args[1])?;

                self.write_register(register, a)?
            }
            Operation::Jmp => {
                let value = self.argument_to_float(&args[0])?;

                self.write_register(RegisterKind::ProgramCounter, value)?
            }
            Operation::Dbg => {
                let Some(callback) = &self.dbg_callback else {
                    return Ok(None);
                };

                let value = self.argument_to_float(&args[0])?;

                callback(value.to_string())
            }
            Operation::Dbgs => {
                todo!()
            }
            Operation::Yield => return Ok(Some(VMStatus::Yield)),
            Operation::Beq
            | Operation::Bge
            | Operation::Bgt
            | Operation::Ble
            | Operation::Blt
            | Operation::Bne => {
                let a = self.argument_to_float(&args[0])?;
                let b = self.argument_to_float(&args[1])?;

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
                        self.argument_to_float(&args[2])?,
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
                        self.argument_to_float(&args[1])?,
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
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };
                let a = self.argument_to_float(&args[1])?;
                let b = self.argument_to_float(&args[2])?;

                let result = match operation {
                    Operation::Seq => a == b,
                    Operation::Sge => a >= b,
                    Operation::Sgt => a > b,
                    Operation::Sle => a <= b,
                    Operation::Slt => a < b,
                    Operation::Sne => a != b,
                    _ => unreachable!(),
                };

                self.write_register(*register, if result { 1.0 } else { 0.0 })?
            }
            Operation::Seqz
            | Operation::Sgez
            | Operation::Sgtz
            | Operation::Slez
            | Operation::Sltz
            | Operation::Snez => {
                let Argument::Register { register } = &args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
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

                self.write_register(*register, if result { 1.0 } else { 0.0 })?
            }
            Operation::Halt => return Ok(Some(VMStatus::Finished)),
            Operation::Push => {
                self.push_stack(self.argument_to_float(&args[0])?)?;
            }
            Operation::Pop => {
                let Argument::Register { register } = &args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };

                let a = self.pop_stack()?;
                self.write_register(*register, a)?;
            }
            Operation::Peek => {
                let Argument::Register { register } = &args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };

                let ret = self.peek_stack()?;
                self.write_register(*register, ret)?;
            }
            Operation::Ret => {
                let ret = self.pop_stack()?;

                self.write_register(RegisterKind::ProgramCounter, ret)?;
            }
            Operation::Call => {
                let a = self.argument_to_float(&args[0])?;

                self.push_stack(self.register_to_float(RegisterKind::ProgramCounter)? + 1.0)?;
                self.write_register(RegisterKind::ProgramCounter, a)?
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
                let Argument::Register { register } = &args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };
                let a = self.argument_to_float(&args[1])? as i32;
                let b = self.argument_to_float(&args[2])? as i32;

                let result = match operation {
                    Operation::And => {
                        if (a != 0) && (b != 0) {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Or => {
                        if (a != 0) || (b != 0) {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Xor => {
                        if (a != 0) ^ (b != 0) {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Nor => {
                        if (a == 0) && (b == 0) {
                            1
                        } else {
                            0
                        }
                    }
                    Operation::Andi => a & b,
                    Operation::Ori => a | b,
                    Operation::Xori => a ^ b,
                    Operation::Shr => a.wrapping_shr(b as u32),
                    Operation::Shl => a.wrapping_shl(b as u32),
                    Operation::Ror => a.rotate_right(b as u32),
                    Operation::Rol => a.rotate_left(b as u32),
                    _ => unreachable!(),
                };

                self.write_register(*register, result as f32)?;
            }
            Operation::Sqrt
            | Operation::Trunc
            | Operation::Ceil
            | Operation::Floor
            | Operation::Abs
            | Operation::Exp
            | Operation::Inf
            | Operation::Nan => {
                let Argument::Register { register } = &args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };
                let a = self.argument_to_float(&args[1])?;

                let result = match operation {
                    Operation::Sqrt => a.sqrt(),
                    Operation::Trunc => a.trunc(),
                    Operation::Ceil => a.ceil(),
                    Operation::Floor => a.floor(),
                    Operation::Abs => a.abs(),
                    Operation::Exp => a.exp(),
                    Operation::Inf => {
                        if a.is_infinite() {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    Operation::Nan => {
                        if a.is_nan() {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    _ => unreachable!(),
                };

                self.write_register(*register, result)?;
            }
            Operation::Max | Operation::Min | Operation::Log => {
                let Argument::Register { register } = &args[0] else {
                    return Err(RuntimeError::new(
                        String::from("Expected register"),
                        RuntimeErrorKind::InvalidType,
                    ));
                };
                let a = self.argument_to_float(&args[1])?;
                let b = self.argument_to_float(&args[2])?;

                let result = match operation {
                    Operation::Max => f32::max(a, b),
                    Operation::Min => f32::min(a, b),
                    Operation::Log => f32::log(b, a),
                    _ => unreachable!(),
                };

                self.write_register(*register, result)?;
            }
        }

        Ok(None)
    }

    fn register_to_float(&self, register: RegisterKind) -> Result<f32, RuntimeError> {
        match register {
            RegisterKind::Regular { id, mode } => match mode {
                RegisterMode::Direct => Ok(self.registers[id]),
                RegisterMode::Indirect => Ok(self.register_to_float(RegisterKind::Regular {
                    id: self.registers[id] as usize,
                    mode: RegisterMode::Direct,
                })?),
                RegisterMode::Address => todo!(),
            },
            RegisterKind::ProgramCounter => Ok(self.pc as f32),
            RegisterKind::StackPointer => Ok(self.sp as f32),
            RegisterKind::Pin { .. } => todo!(),
        }
    }

    fn argument_to_float(&self, argument: &Argument) -> Result<f32, RuntimeError> {
        match argument {
            Argument::Register { register: kind } => self.register_to_float(*kind),
            Argument::Int { value } => Ok(*value as f32),
            Argument::Float { value } => Ok(*value),
            _ => Err(RuntimeError::new(
                format!("Argument {argument} can't be used as value"),
                RuntimeErrorKind::InvalidType,
            )),
        }
    }
}
