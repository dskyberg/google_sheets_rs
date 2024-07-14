use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use thiserror::Error;

use crate::{DeveloperMetadata, Sheet};

use super::{DataSource, DataSourceRefreshSchedule, NamedRange, SpreadsheetProperties};

#[derive(Debug, Error)]
pub enum SpreadsheetError {
    #[error("No sheets")]
    NoSheets,
    #[error("Sheet name not found {0}")]
    SheetNotFound(String),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spreadsheet {
    pub spreadsheet_id: Option<String>,
    pub properties: Option<SpreadsheetProperties>,
    pub sheets: Option<Vec<Sheet>>,
    pub named_ranges: Option<Vec<NamedRange>>,
    pub spreadsheet_url: Option<String>,
    pub developer_metadata: Option<Vec<DeveloperMetadata>>,
    pub data_sources: Option<Vec<DataSource>>,
    pub data_source_schedules: Option<Vec<DataSourceRefreshSchedule>>,
}

impl Spreadsheet {
    pub fn new() -> Self {
        Self {
            spreadsheet_id: None,
            properties: None,
            sheets: None,
            named_ranges: None,
            spreadsheet_url: None,
            developer_metadata: None,
            data_sources: None,
            data_source_schedules: None,
        }
    }

    pub fn find_sheet(&self, name: &str) -> Option<&Sheet> {
        let sheets = self.sheets.as_ref()?;
        for sheet in sheets {
            let Some(props) = sheet.properties.as_ref() else {
                continue;
            };
            if let Some(title) = &props.title {
                if title == name {
                    return Some(sheet);
                }
            }
        }
        None
    }

    pub fn find_sheet_mut(&mut self, name: &str) -> Option<&mut Sheet> {
        let sheets = self.sheets.as_mut()?;
        for sheet in sheets {
            let Some(props) = sheet.properties.as_ref() else {
                continue;
            };
            if let Some(title) = &props.title {
                if title == name {
                    return Some(sheet);
                }
            }
        }
        None
    }

    /// Get a sheet by name
    pub fn get_sheet(&self, name: &str) -> Result<&Sheet> {
        self.find_sheet(name)
            .ok_or(SpreadsheetError::SheetNotFound(name.to_string()).into())
    }

    /// Get a sheet by name
    pub fn get_sheet_mut(&mut self, name: &str) -> Result<&mut Sheet> {
        self.find_sheet_mut(name)
            .ok_or(SpreadsheetError::SheetNotFound(name.to_string()).into())
    }
}

impl Default for Spreadsheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spreadsheet() {
        let json = include_str!("../../../../response.json");
        let mut spreadsheet = serde_json::from_str::<Spreadsheet>(json).expect("oops");
        let sheet = spreadsheet.get_sheet_mut("Accounts").expect("oops");
        sheet.read_headers().expect("Failed headers");

        dbg!(sheet.headers.as_ref());
    }
}
