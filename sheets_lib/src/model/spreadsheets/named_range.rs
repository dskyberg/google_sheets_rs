use serde::{Deserialize, Serialize};

use crate::GridRange;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRange {
    pub named_range_id: String,
    pub name: String,
    pub range: GridRange,
}
