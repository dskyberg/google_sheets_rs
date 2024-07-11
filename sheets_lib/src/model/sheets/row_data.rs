use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::CellData;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowData {
    pub values: Vec<CellData>,
}
