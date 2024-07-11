use serde::{Deserialize, Serialize};

use crate::BooleanCondition;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataValidationRule {
    pub condition: BooleanCondition,
    pub input_message: String,
    pub strict: bool,
    pub show_custom_ui: bool,
}
