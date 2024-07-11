use serde::{Deserialize, Serialize};

use super::{ConditionType, ConditionValue};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanCondition {
    #[serde(rename = "type")]
    pub _type: ConditionType,
    pub values: Vec<ConditionValue>,
}
