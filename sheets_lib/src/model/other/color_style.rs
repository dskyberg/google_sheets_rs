use serde::{Deserialize, Serialize};

use super::{Color, ThemeColorType};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorStyle {
    // Union field kind can be only one of the following:
    pub rgb_color: Color,
    pub theme_color: Option<ThemeColorType>, // End of list of possible types for union field kind.
}
