use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::DimensionType;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionRange {
    pub sheet_id: u32,
    pub dimension: DimensionType,
    pub start_index: u32,
    pub end_index: u32,
}
