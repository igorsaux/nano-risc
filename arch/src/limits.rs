use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    #[serde(alias = "regularRegisters")]
    pub regular_registers: usize,
    pub pins: usize,
    #[serde(alias = "stackSize")]
    pub stack_size: usize,
    #[serde(alias = "ramLength")]
    pub ram_length: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            regular_registers: 16,
            pins: 8,
            stack_size: 256,
            ram_length: 16384,
        }
    }
}
