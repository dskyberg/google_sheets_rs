use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{CellData, Value};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RowData {
    pub values: Vec<CellData>,
}

impl RowData {
    pub fn as_values(&self) -> Vec<Value> {
        let mut result = Vec::<Value>::new();

        for cell_data in &self.values {
            if let Some(value) = cell_data.effective_value.as_ref() {
                result.push(Value::from(value));
            } else if let Some(value) = cell_data.formatted_value.as_ref() {
                result.push(Value::StringValue(value.to_string()));
            } else {
                result.push(Value::StringValue(String::from("")));
            }
        }
        result
    }
}
