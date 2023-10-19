use nano_risc_arch::Location;
use serde::{Deserialize, Serialize};

use super::CompilationErrorKind;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilationError {
    message: String,
    location: Location,
    kind: CompilationErrorKind,
}

impl CompilationError {
    pub fn new(message: String, location: Location, kind: CompilationErrorKind) -> Self {
        Self {
            message,
            location,
            kind,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn kind(&self) -> &CompilationErrorKind {
        &self.kind
    }
}
