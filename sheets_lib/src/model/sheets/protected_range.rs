use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::Editors;
use crate::GridRange;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtectedRange {
    pub protected_range_id: u32,
    pub range: Option<GridRange>,
    pub named_range_id: Option<String>,
    pub description: Option<String>,
    pub warning_only: bool,
    pub requesting_user_can_edit: bool,
    pub unprotected_ranges: Option<Vec<GridRange>>,
    pub editors: Option<Editors>,
}
