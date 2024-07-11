use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SheetType {
    SheetTypeUnspecified,
    Grid,
    Object,
    DataSource,
}
