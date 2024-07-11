use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{FilterSpec, GridRange, SortSpec};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicFilter {
    pub range: GridRange,
    pub sort_specs: Option<Vec<SortSpec>>,
    pub filter_specs: Vec<FilterSpec>,
}
