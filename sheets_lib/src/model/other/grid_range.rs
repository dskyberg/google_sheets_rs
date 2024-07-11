use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridRange {
    pub sheet_id: Option<u32>,
    pub start_row_index: Option<u32>,
    pub end_row_index: Option<u32>,
    pub start_column_index: Option<u32>,
    pub end_column_index: Option<u32>,
}
