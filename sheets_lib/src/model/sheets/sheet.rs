use anyhow::Result;
use std::collections::BTreeMap;
use thiserror::Error;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DeveloperMetadata, GridRange};

use super::{
    BandedRange, BasicFilter, ConditionalFormatRule, DimensionGroup, FilterView, GridData,
    ProtectedRange, SheetProperties, Slicer,
};

#[derive(Debug, Error)]
pub enum SheetError {
    #[error("No Data")]
    NoData,
    #[error("data exists, but is empty")]
    EmptyData,
    #[error("Empty row")]
    EmptyRow,
    #[error("No properties")]
    NoProperties,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    #[serde(skip)]
    pub headers: Option<BTreeMap<String, usize>>,
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

impl Sheet {
    pub fn new() -> Self {
        Self {
            headers: None,
            properties: None,
            data: None,
            merges: None,
            conditional_formats: None,
            filter_views: None,
            protected_ranges: None,
            basic_filter: None,
            banded_ranges: None,
            developer_metadata: None,
            row_groups: None,
            column_groups: None,
            slicers: None,
        }
    }

    pub fn get_title(&self) -> Option<&String> {
        self.properties.as_ref()?.title.as_ref()
    }

    /// Generally, expect Vec<GridData> to have 1 entry
    #[inline]
    pub fn get_data(&self) -> Result<&GridData, SheetError> {
        self.data
            .as_ref()
            .ok_or(SheetError::EmptyData)?
            .first()
            .ok_or(SheetError::EmptyData)
    }

    pub fn get_properties(&self) -> Result<&SheetProperties> {
        self.properties
            .as_ref()
            .ok_or(SheetError::NoProperties.into())
    }

    pub fn read_headers(&mut self) -> Result<()> {
        // Don't run multiple times
        if self.headers.is_some() {
            return Ok(());
        }

        let mut headers = BTreeMap::<String, usize>::new();
        let grid_data = self.get_data()?;
        let row_data = grid_data.get_data()?.first().ok_or(SheetError::NoData)?;

        for (idx, cell_data) in row_data.values.iter().enumerate() {
            let header = cell_data
                .formatted_value
                .as_ref()
                .ok_or(SheetError::EmptyRow)?;
            headers.insert(header.to_owned(), idx);
        }

        self.headers = Some(headers);
        Ok(())
    }
}

impl Default for Sheet {
    fn default() -> Self {
        Self::new()
    }
}
