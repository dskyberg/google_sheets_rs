use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DeveloperMetadata, Sheet};

use super::{DataSource, DataSourceRefreshSchedule, NamedRange, SpreadsheetProperties};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spreadsheet() {
        let json = include_str!("../../../../response.json");
        let result = serde_json::from_str::<Spreadsheet>(json).expect("oops");
        dbg!(result);
    }
}
