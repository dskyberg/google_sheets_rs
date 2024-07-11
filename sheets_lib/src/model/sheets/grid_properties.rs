use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridProperties {
    pub row_count: u32,
    pub column_count: u32,
    pub frozen_row_rount: Option<u32>,
    pub frozen_column_count: Option<u32>,
    pub hide_gridlines: Option<bool>,
    pub row_group_control_after: Option<bool>,
    pub column_group_control_after: Option<bool>,
}
