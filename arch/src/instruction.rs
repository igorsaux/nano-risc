use crate::{Argument, Operation};

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub operation: Operation,
    pub arguments: Vec<Argument>,
}
