use serde::{Deserialize, Serialize};

use crate::{ColorStyle, ThemeColorType};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeColorPair {
    pub color_type: ThemeColorType,
    pub color: ColorStyle,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetTheme {
    pub primary_font_family: Option<String>,
    pub theme_colors: Option<Vec<ThemeColorPair>>,
}
