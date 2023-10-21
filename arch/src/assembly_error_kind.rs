use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssemblyErrorKind {
    InvalidInstruction { name: String },
    InvalidRegister { id: usize },
    InvalidPin { id: usize },
    TooLarge,
}
