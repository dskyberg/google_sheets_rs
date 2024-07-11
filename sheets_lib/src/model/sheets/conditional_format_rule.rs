use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::GridRange;

use super::{BooleanRule, GradientRule};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalFormatRule {
    pub ranges: Option<Vec<GridRange>>,
    // Union field rule can be only one of the following:
    pub boolean_rule: Option<BooleanRule>,
    pub gradient_rule: Option<GradientRule>, // End of list of possible types for union field rule.
}
