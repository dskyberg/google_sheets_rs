use serde::{Deserialize, Serialize};

use crate::{BooleanCondition, CellFormat};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BooleanRule {
    pub condition: BooleanCondition,
    pub format: CellFormat,
}
