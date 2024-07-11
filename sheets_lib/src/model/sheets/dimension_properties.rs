use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DataSourceColumnReference, DeveloperMetadata};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionProperties {
    pub hidden_by_filter: Option<bool>,
    pub hidden_by_user: Option<bool>,
    pub pixel_size: u32,
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    pub data_source_column_reference: Option<DataSourceColumnReference>,
}
