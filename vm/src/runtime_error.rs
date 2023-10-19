use std::fmt::Display;

use arch::RegisterKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidType { message: String },
    DividedByZero,
    RegisterIsReadOnly { register: RegisterKind },
    InvalidRegister { register: RegisterKind },
    InvalidPosition { position: usize },
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::InvalidType { message } => {
                f.write_fmt(format_args!("InvalidType: {message}"))
            }
            RuntimeError::DividedByZero => f.write_fmt(format_args!("DivideByZero")),
            RuntimeError::RegisterIsReadOnly { register } => f.write_fmt(format_args!(
                "RegisterIsReadOnly: The register {register} is read only"
            )),
            RuntimeError::InvalidRegister { register } => {
                f.write_fmt(format_args!("InvalidRegister: {register}"))
            }
            RuntimeError::InvalidPosition { position } => f.write_fmt(format_args!(
                "InvalidPosition: current PC is out of bounds: {position}"
            )),
        }
    }
}
