use crate::{Argument, Operation};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Comment {
        text: String,
    },
    Label {
        name: String,
    },
    Instruction {
        operation: Operation,
        args: Vec<Argument>,
    },
}
