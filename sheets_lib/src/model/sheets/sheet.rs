use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DeveloperMetadata, GridRange};

use super::{
    BandedRange, BasicFilter, ConditionalFormatRule, DimensionGroup, FilterView, GridData,
    ProtectedRange, SheetProperties, Slicer,
};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    pub properties: Option<SheetProperties>,
    pub data: Option<Vec<GridData>>,
    pub merges: Option<Vec<GridRange>>,
    pub conditional_formats: Option<Vec<ConditionalFormatRule>>,
    pub filter_views: Option<Vec<FilterView>>,
    pub protected_ranges: Option<Vec<ProtectedRange>>,
    pub basic_filter: Option<BasicFilter>,
    pub banded_ranges: Option<Vec<BandedRange>>,
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    pub row_groups: Option<Vec<DimensionGroup>>,
    pub column_groups: Option<Vec<DimensionGroup>>,
    pub slicers: Option<Vec<Slicer>>,
}
