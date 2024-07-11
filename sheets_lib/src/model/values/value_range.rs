use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::DimensionType;

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

impl ValueInputOption {
    pub fn to_query(&self) -> String {
        format!("valueInputOption={}", self)
    }
}

/// JSON structure for sheet downloaded from Google
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueRange {
    pub range: String,
    pub major_dimension: DimensionType,
    pub values: Vec<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let value_input_option = ValueInputOption::InputValueOptionUnspecified;

        println!("{}", &value_input_option.to_query());
    }
}
