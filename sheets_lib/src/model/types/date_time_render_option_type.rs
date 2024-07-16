use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateTimeRenderOptionType {
    SerialNumber,
    FormattedString,
}

impl std::fmt::Display for DateTimeRenderOptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_case(Case::ScreamingSnake))
    }
}
