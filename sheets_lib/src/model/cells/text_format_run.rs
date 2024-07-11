use serde::{Deserialize, Serialize};

use crate::TextFormat;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextFormatRun {
    pub start_index: u32,
    pub format: TextFormat,
}
