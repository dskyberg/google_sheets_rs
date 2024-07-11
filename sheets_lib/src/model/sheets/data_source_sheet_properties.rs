use serde::{Deserialize, Serialize};

use crate::{DataExecutionStatus, DataSourceColumn};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSheetProperties {
    pub data_source_id: String,
    pub columns: Vec<DataSourceColumn>,
    pub data_execution_status: DataExecutionStatus,
}
