use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{ColorStyle, DataSourceColumnReference, SortOrderType};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SortSpec {
    pub sort_order: SortOrderType,
    pub foreground_color_style: Option<ColorStyle>,
    pub background_color_style: Option<ColorStyle>,
    // Union field reference can be only one of the following:
    pub dimension_lndex: Option<u32>,
    pub data_source_column_reference: Option<DataSourceColumnReference>, // End of list of possible types for union field reference.
}
