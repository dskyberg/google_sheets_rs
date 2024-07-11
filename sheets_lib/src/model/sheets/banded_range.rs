use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::GridRange;

use super::BandingProperties;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BandedRange {
    pub banded_range_id: u32,
    pub range: GridRange,
    pub row_properties: Option<BandingProperties>,
    pub columnproperties: Option<BandingProperties>,
}
