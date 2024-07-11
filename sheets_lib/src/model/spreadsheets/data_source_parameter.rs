use serde::{Deserialize, Serialize};

use crate::GridRange;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceParameter {
    // Union field identifier can be only one of the following:
    pub name: Option<String>,
    // End of list of possible types for union field identifier.

    // Union field value can be only one of the following:
    pub named_range_id: Option<String>,
    pub range: Option<GridRange>, // End of list of possible types for union field value.
}
