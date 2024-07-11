use serde::{Deserialize, Serialize};

use super::DataSourceColumnReference;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DataSourceColumn {
    pub reference: Option<DataSourceColumnReference>,
    pub formula: Option<String>,
}
