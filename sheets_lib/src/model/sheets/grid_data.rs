use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use thiserror::Error;

use super::{DimensionProperties, RowData};

#[derive(Debug, Error)]
pub enum GridDataError {
    #[error("Data was expected")]
    NoData,
    #[error("Start row was expected")]
    NoStartRow,
    #[error("Start column was expected")]
    NoStartColumn,
    #[error("No row metadata")]
    NoRowMetadata,
    #[error("No column metadata")]
    NoColumnMetadata,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GridData {
    pub start_row: Option<u32>,
    pub start_column: Option<u32>,
    pub row_data: Option<Vec<RowData>>,
    pub row_metadata: Option<Vec<DimensionProperties>>,
    pub column_metadata: Option<Vec<DimensionProperties>>,
}

impl GridData {
    pub fn new() -> Self {
        Self {
            start_row: None,
            start_column: None,
            row_data: None,
            row_metadata: None,
            column_metadata: None,
        }
    }

    pub fn get_data(&self) -> Result<&Vec<RowData>, GridDataError> {
        self.row_data.as_ref().ok_or(GridDataError::NoData)
    }

    pub fn get_start_row(&self) -> Result<u32, GridDataError> {
        self.start_row.ok_or(GridDataError::NoStartRow)
    }

    pub fn get_start_column(&self) -> Result<u32, GridDataError> {
        self.start_column.ok_or(GridDataError::NoStartColumn)
    }
    pub fn get_row_metadata(&self) -> Result<&Vec<DimensionProperties>, GridDataError> {
        self.row_metadata
            .as_ref()
            .ok_or(GridDataError::NoRowMetadata)
    }

    pub fn get_column_metadata(&self) -> Result<&Vec<DimensionProperties>, GridDataError> {
        self.column_metadata
            .as_ref()
            .ok_or(GridDataError::NoColumnMetadata)
    }
}

impl Default for GridData {
    fn default() -> Self {
        Self::new()
    }
}
