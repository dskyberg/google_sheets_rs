use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{DataSourceColumnReference, FilterCriteria};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterSpec {
    pub filter_criteria: FilterCriteria,

    // Union field reference can be only one of the following:
    pub column_index: Option<u32>,
    pub data_source_column_reference: Option<DataSourceColumnReference>,
    // End of list of possible types for union field reference.
}
