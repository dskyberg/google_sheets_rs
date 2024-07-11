use serde::{Deserialize, Serialize};

use super::NumberFormatType;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberFormat {
    #[serde(rename = "type")]
    pub _type: Option<NumberFormatType>,
    pub pattern: Option<String>,
}
