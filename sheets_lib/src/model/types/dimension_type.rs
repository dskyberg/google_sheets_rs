use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DimensionType {
    DimensionUnspecified,
    Rows,
    Columns,
}

impl std::fmt::Display for DimensionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_case(Case::ScreamingSnake))
    }
}
