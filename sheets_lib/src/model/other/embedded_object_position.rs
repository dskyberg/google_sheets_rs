use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::OverlayPosition;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct EmbeddedObjectPosition {
    // Union field location can be only one of the following:
    pub sheet_id: Option<u32>,
    pub overlay_position: Option<OverlayPosition>,
    pub new_sheet: Option<bool>, // End of list of possible types for union field location.
}
