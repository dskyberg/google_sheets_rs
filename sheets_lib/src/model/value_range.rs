use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{DimensionType, GridData, RowData, Sheet};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValueInputOption {
    InputValueOptionUnspecified,
    Raw,
    UserEntered,
}

impl Display for ValueInputOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{:?}", self).to_case(Case::UpperSnake);
        write!(f, "{}", value)
    }
}

impl Default for ValueInputOption {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueInputOption {
    pub fn new() -> Self {
        Self::Raw
    }

    pub fn to_query(&self) -> String {
        format!("valueInputOption={}", self)
    }
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseValueRenderOption {
    FormattedValue,
    UnformattedValue,
    Formula,
}

impl std::fmt::Display for ResponseValueRenderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{:?}", self).to_case(Case::UpperSnake);
        write!(f, "{}", value)
    }
}
impl Default for ResponseValueRenderOption {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseValueRenderOption {
    pub fn new() -> Self {
        Self::UnformattedValue
    }

    pub fn to_query(&self) -> String {
        format!("responseValueRenderOption={}", &self)
    }
}

/// JSON structure for sheet downloaded from Google
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Value {
    NumberValue(f64), //	double	Represents a double value. Note that attempting to serialize NaN or Infinity results in error. (We can't serialize these as string "NaN" or "Infinity" values like we do for regular fields, because they would parse as string_value, not number_value).
    StringValue(String), //	Represents a string value.
    BoolValue(bool),  // Represents a boolean value.
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::NumberValue(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::BoolValue(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<String> {
        match self {
            Self::StringValue(s) => Some(s.to_owned()),
            _ => None,
        }
    }
}

/// JSON structure for sheet downloaded from Google
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    pub range: String,
    pub major_dimension: DimensionType,
    pub values: Vec<Vec<Value>>,
}

impl From<&Sheet> for ValueRange {
    fn from(sheet: &Sheet) -> Self {
        let major_dimension = DimensionType::Rows;
        let range = match sheet.get_title() {
            Some(r) => r.to_string(),
            None => "".to_string(),
        };

        let empty = GridData::new();
        let empty_vec = Vec::<RowData>::new();

        let values = sheet
            .get_data()
            .unwrap_or(&empty)
            .get_data()
            .unwrap_or(&empty_vec)
            .iter()
            .map(|row| row.as_values())
            .collect::<Vec<Vec<Value>>>();
        ValueRange {
            range,
            major_dimension,
            values,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_querys() {
        let vro = ResponseValueRenderOption::UnformattedValue;
        println!("{}", vro.to_query())
    }
    #[test]
    fn test_deserialize_value_range() {
        let json = include_str!("../../../value_range_response.json");
        let value_range = serde_json::from_str::<ValueRange>(json).expect("Failed to deserialize");
        let rendered = serde_json::to_string_pretty(&value_range).expect("Failed to serialize");

        println!("{}", rendered);
    }
    #[test]
    fn test_query() {
        let value_input_option = ValueInputOption::InputValueOptionUnspecified;

        println!("{}", &value_input_option.to_query());
    }
}
