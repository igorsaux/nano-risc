#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidType { message: String },
    DividedByZero,
}
