use nano_risc_arch::RegisterKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeErrorKind {
    InvalidType,
    DividedByZero,
    RegisterIsReadOnly { register: RegisterKind },
    InvalidRegister { register: RegisterKind },
    InvalidAddress { address: usize },
    StackOverflow,
}
