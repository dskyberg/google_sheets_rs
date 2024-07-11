use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{DimensionProperties, RowData};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridData {
    pub start_row: Option<u32>,
    pub start_column: Option<u32>,
    pub row_data: Option<Vec<RowData>>,
    pub row_metadata: Option<Vec<DimensionProperties>>,
    pub column_metadata: Option<Vec<DimensionProperties>>,
}
