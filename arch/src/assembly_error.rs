use serde::{Deserialize, Serialize};

use crate::{AssemblyErrorKind, Location};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssemblyError {
    message: String,
    location: Option<Location>,
    kind: AssemblyErrorKind,
}

impl AssemblyError {
    pub fn new(message: String, location: Option<Location>, kind: AssemblyErrorKind) -> Self {
        Self {
            message,
            location,
            kind,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn location(&self) -> Option<Location> {
        self.location
    }

    pub fn kind(&self) -> &AssemblyErrorKind {
        &self.kind
    }
}
