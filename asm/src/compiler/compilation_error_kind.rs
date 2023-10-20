use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationErrorKind {
    InvalidOperation,
    InvalidArgument,
    DuplicateLabel { name: String },
    UnknownLabel { name: String },
    TooLargeAssembly { size: usize },
}
