use serde::{Deserialize, Serialize};

use super::{DeveloperMetadataLocation, DeveloperMetadataVisibilityType};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperMetadata {
    pub metadata_id: u32,
    pub metadata_key: String,
    pub metadata_value: String,
    pub location: DeveloperMetadataLocation,
    pub visibility: DeveloperMetadataVisibilityType,
}
