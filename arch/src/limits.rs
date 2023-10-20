use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    #[serde(alias = "regularRegisters")]
    pub regular_registers: usize,
    pub pins: usize,
    #[serde(alias = "maxAssemblyLength")]
    pub max_assembly_length: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            regular_registers: 16,
            pins: 8,
            max_assembly_length: 4096,
        }
    }
}
