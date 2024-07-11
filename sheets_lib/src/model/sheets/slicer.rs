use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{ColorStyle, FilterCriteria, GridRange, HorizontalAlignType, TextFormat};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Slicer {
    pub data_range: Option<GridRange>,
    pub filter_criteria: Option<FilterCriteria>,
    pub column_index: Option<u32>,
    pub apply_to_pivot_tables: Option<bool>,
    pub title: Option<String>,
    pub text_format: Option<TextFormat>,
    pub background_color_style: Option<ColorStyle>,
    pub horizontal_alignment: Option<HorizontalAlignType>,
}
