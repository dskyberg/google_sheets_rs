use serde::{Deserialize, Serialize};

use super::ErrorType;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorValue {
    #[serde(rename = "type")]
    pub _type: ErrorType,
    pub message: Option<String>,
}
