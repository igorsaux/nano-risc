use crate::{Location, SourceUnit};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugInfo {
    #[serde(alias = "sourceLoc")]
    pub source_loc: BTreeMap<usize, Location>,
    pub unit: SourceUnit,
}
