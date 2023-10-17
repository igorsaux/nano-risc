use arch::Operation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    DuplicatedLabel {
        name: String,
    },
    InvalidLabel {
        name: String,
    },
    InvalidInstruction {
        operation: Operation,
        message: String,
    },
    InvalidRegister {
        id: usize,
    },
    InvalidPin {
        id: usize,
    },
}
