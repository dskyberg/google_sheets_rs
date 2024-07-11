use serde::{Deserialize, Serialize};

use crate::{Color, ColorStyle, TextFormat};

use super::{
    Borders, HorizontalAlignType, HyperlinkDisplayType, NumberFormat, Padding, TextDirectionType,
    TextRotation, VerticalAlignType, WrapStrategyType,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CellFormat {
    pub number_format: Option<NumberFormat>,
    pub background_color: Option<Color>,
    pub background_color_style: Option<ColorStyle>,
    pub borders: Option<Borders>,
    pub padding: Option<Padding>,
    pub horizontal_alignment: Option<HorizontalAlignType>,
    pub vertical_alignment: Option<VerticalAlignType>,
    pub wrap_strategy: Option<WrapStrategyType>,
    pub text_direction: Option<TextDirectionType>,
    pub text_format: Option<TextFormat>,
    pub hyperlink_display_type: Option<HyperlinkDisplayType>,
    pub text_rotation: Option<TextRotation>,
}
