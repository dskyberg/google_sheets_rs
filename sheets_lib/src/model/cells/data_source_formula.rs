use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::DataExecutionStatus;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceFormula {
    pub data_source_id: String,
    pub data_execution_status: Option<DataExecutionStatus>,
}
