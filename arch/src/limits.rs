use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    #[serde(alias = "regularRegisters")]
    pub regular_registers: usize,
    pub pins: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            regular_registers: 8,
            pins: 6,
        }
    }
}
