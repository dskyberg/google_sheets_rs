use serde::{Deserialize, Serialize};

use crate::{Color, ColorStyle, StyleType};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Border {
    pub style: StyleType,
    pub width: u32,
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
}
