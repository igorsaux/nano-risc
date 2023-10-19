use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

impl Location {
    pub fn new(line: usize, row: usize, offset: usize) -> Self {
        Self {
            line,
            column: row,
            offset,
        }
    }
}
