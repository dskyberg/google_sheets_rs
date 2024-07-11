use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DataSourceColumnReference {
    pub name: String,
}
