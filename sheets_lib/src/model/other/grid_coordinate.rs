use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GridCoordinate {
    pub sheet_id: u32,
    pub row_index: u32,
    pub column_index: u32,
}
