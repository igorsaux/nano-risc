use std::fmt::Display;

use crate::RegisterKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Argument {
    Register { register: RegisterKind },
    Pin { id: usize },
    Int { value: i32 },
    Float { value: f32 },
}

impl Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Register { register } => Display::fmt(register, f),
            Argument::Pin { id } => f.write_fmt(format_args!("p{id}")),
            Argument::Int { value } => Display::fmt(value, f),
            Argument::Float { value } => Display::fmt(value, f),
        }
    }
}
