use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{FilterSpec, GridRange, SortSpec};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterView {
    pub filter_view_id: u32,
    pub title: Option<String>,
    pub range: Option<GridRange>,
    pub named_range_id: Option<String>,
    pub sort_specs: Option<Vec<SortSpec>>,
    pub filter_specs: Option<Vec<FilterSpec>>,
}
