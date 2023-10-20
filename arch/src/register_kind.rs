use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterKind {
    Regular { id: usize },
    ProgramCounter,
    StackPointer,
}

impl Display for RegisterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterKind::Regular { id } => f.write_fmt(format_args!("R{id}")),
            RegisterKind::ProgramCounter => f.write_fmt(format_args!("PC")),
            RegisterKind::StackPointer => f.write_fmt(format_args!("SP")),
        }
    }
}
