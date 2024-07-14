use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::ErrorValue;
use crate::Value;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedValue {
    // Union field value can be only one of the following:
    pub number_value: Option<f64>,
    pub string_value: Option<String>,
    pub bool_value: Option<bool>,
    pub formula_value: Option<String>,
    pub error_value: Option<ErrorValue>, // End of list of possible types for union field value.
}

impl From<&ExtendedValue> for Value {
    fn from(value: &ExtendedValue) -> Self {
        if let Some(n) = value.number_value.as_ref() {
            return Value::NumberValue(*n);
        }
        if let Some(s) = value.string_value.as_ref() {
            return Value::StringValue(s.to_string());
        }
        if let Some(b) = value.bool_value.as_ref() {
            return Value::BoolValue(*b);
        }
        // Enpty values become empty strings
        Value::StringValue("".to_string())
    }
}
