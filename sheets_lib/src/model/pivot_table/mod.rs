use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{DataExecutionStatus, GridRange};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotGroup {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotFilterSpec {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotValue {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PivotValueLayoutType {}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PivotTable {
    pub rows: Vec<PivotGroup>,
    pub columns: Vec<PivotGroup>,
    pub filter_specs: Vec<PivotFilterSpec>,
    pub values: Vec<PivotValue>,
    pub value_layout: PivotValueLayoutType,
    pub data_execution_status: DataExecutionStatus,

    // Union field source_data can be only one of the following:
    pub source: Option<GridRange>,
    data_source_id: Option<String>, // End of list of possible types for union field source_data.
}
