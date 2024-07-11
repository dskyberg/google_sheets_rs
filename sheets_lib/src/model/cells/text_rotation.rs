use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRotation {
    // Union field type can be only one of the following:
    pub angle: Option<u32>,
    pub vertical: Option<bool>, // End of list of possible types for union field type.
}
