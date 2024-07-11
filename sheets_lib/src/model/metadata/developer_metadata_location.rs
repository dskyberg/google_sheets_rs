use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{DeveloperMetadataLocationType, DimensionRange};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadataLocation {
    pub location_type: DeveloperMetadataLocationType,

    // Union field location can be only one of the following:
    pub spreadsheet: Option<bool>,
    pub sheet_id: Option<u32>,
    pub dimension_range: Option<DimensionRange>, // End of list of possible types for union field location.
}
