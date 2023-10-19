use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    #[serde(alias = "regularRegisters")]
    pub regular_registers: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            regular_registers: 16,
        }
    }
}
