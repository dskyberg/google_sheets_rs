use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::DimensionRange;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionGroup {
    pub range: DimensionRange,
    pub depth: u32,
    pub collapsed: bool,
}
