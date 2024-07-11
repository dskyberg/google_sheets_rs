use serde::{Deserialize, Serialize};

use super::DataSourceSpec;

use crate::DataSourceColumn;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
    pub data_source_id: String,
    pub spec: DataSourceSpec,
    pub calculated_columns: Vec<DataSourceColumn>,
    pub sheet_id: u32,
}
