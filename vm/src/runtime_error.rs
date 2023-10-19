use arch::RegisterKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidType { message: String },
    DividedByZero,
    RegisterIsReadOnly { register: RegisterKind },
    InvalidRegister { register: RegisterKind },
    InvalidPosition { position: usize },
}
