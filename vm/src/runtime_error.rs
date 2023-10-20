use serde::{Deserialize, Serialize};

use crate::RuntimeErrorKind;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeError {
    message: String,
    kind: RuntimeErrorKind,
}

impl RuntimeError {
    pub fn new(message: String, kind: RuntimeErrorKind) -> Self {
        Self { message, kind }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &RuntimeErrorKind {
        &self.kind
    }
}
