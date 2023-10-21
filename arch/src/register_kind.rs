use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::RegisterMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterKind {
    Regular { id: usize, mode: RegisterMode },
    ProgramCounter,
    StackPointer,
    Pin { id: usize },
}

impl Display for RegisterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterKind::Regular { id, .. } => f.write_fmt(format_args!("R{id}")),
            RegisterKind::ProgramCounter => f.write_str("PC"),
            RegisterKind::StackPointer => f.write_str("SP"),
            RegisterKind::Pin { id } => f.write_fmt(format_args!("D{id}")),
        }
    }
}
