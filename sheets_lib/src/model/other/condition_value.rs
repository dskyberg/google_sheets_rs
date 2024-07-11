use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::RelativeDateType;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionValue {
    // Union field value can be only one of the following:
    pub relative_date: Option<RelativeDateType>,
    pub user_entered_value: Option<String>, // End of list of possible types for union field value.
}
