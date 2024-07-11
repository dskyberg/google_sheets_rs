use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DataExecutionStatus, DataSourceColumnReference, FilterSpec, SortSpec};

use super::DataSourceTableColumnSelectionType;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceTable {
    pub data_source_id: Option<String>,
    pub column_selection_type: Option<DataSourceTableColumnSelectionType>,
    pub columns: Option<Vec<DataSourceColumnReference>>,
    pub filter_specs: Option<Vec<FilterSpec>>,
    pub sort_specs: Option<Vec<SortSpec>>,
    pub row_limit: Option<u32>,
    /// OUTPUT ONLY
    pub data_execution_status: Option<DataExecutionStatus>,
}
