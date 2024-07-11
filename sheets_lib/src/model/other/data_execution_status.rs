use serde::{Deserialize, Serialize};

use super::{DataExecutionErrorCodeType, DataExecutionStateType};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataExecutionStatus {
    pub state: DataExecutionStateType,
    pub error_code: DataExecutionErrorCodeType,
    pub error_message: String,
    pub last_refresh_time: String,
}
