use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Color, ColorStyle};

use super::InterpolationPointType;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterpolationPoint {
    pub color: Option<Color>,
    pub color_style: Option<ColorStyle>,
    #[serde(rename = "type")]
    pub _type: Option<InterpolationPointType>,
    pub value: Option<String>,
}
