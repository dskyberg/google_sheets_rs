use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::ColorStyle;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandingProperties {
    pub header_color_style: Option<ColorStyle>,
    pub first_band_color_style: Option<ColorStyle>,
    pub second_band_color_style: Option<ColorStyle>,
    pub footer_color_style: Option<ColorStyle>,
}
