use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Color, ColorStyle, DataSourceSheetProperties, GridProperties, SheetType};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SheetProperties {
    pub sheet_id: Option<u32>,
    pub title: Option<String>,
    pub index: Option<u32>,
    pub sheet_type: Option<SheetType>,
    pub grid_properties: Option<GridProperties>,
    pub hidden: Option<bool>,
    pub tab_color: Option<Color>,
    pub tab_color_style: Option<ColorStyle>,
    pub right_to_left: Option<bool>,
    pub data_source_sheet_properties: Option<DataSourceSheetProperties>,
}

impl SheetProperties {
    pub fn new() -> Self {
        Self {
            sheet_id: None,
            title: None,
            index: None,
            sheet_type: None,
            grid_properties: None,
            hidden: None,
            tab_color: None,
            tab_color_style: None,
            right_to_left: None,
            data_source_sheet_properties: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}
impl Default for SheetProperties {
    fn default() -> Self {
        Self::new()
    }
}
