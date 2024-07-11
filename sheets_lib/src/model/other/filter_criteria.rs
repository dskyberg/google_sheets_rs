use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{BooleanCondition, ColorStyle};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCriteria {
    pub hidden_values: Option<Vec<String>>,
    pub condition: Option<BooleanCondition>,
    pub visible_background_color_style: Option<ColorStyle>,
    pub visible_foreground_color_style: Option<ColorStyle>,
}
