use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::InterpolationPoint;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradientRule {
    pub minpoint: Option<InterpolationPoint>,
    pub midpoint: Option<InterpolationPoint>,
    pub maxpoint: Option<InterpolationPoint>,
}
