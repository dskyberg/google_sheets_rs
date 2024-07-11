use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Color, ColorStyle, Link};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextFormat {
    pub foreground_color: Option<Color>,
    pub foreground_color_style: Option<ColorStyle>,
    pub font_family: Option<String>,
    pub font_size: Option<u32>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub strikethrough: Option<bool>,
    pub underline: Option<bool>,
    pub link: Option<Link>,
}
