use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{CellFormat, DataSourceFormula, DataSourceTable, DataValidationRule};
use crate::{ExtendedValue, PivotTable, TextFormatRun};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CellData {
    pub user_entered_value: Option<ExtendedValue>,
    pub effective_value: Option<ExtendedValue>,
    pub formatted_value: Option<String>,
    pub user_entered_format: Option<CellFormat>,
    pub effective_format: Option<CellFormat>,
    pub hyperlink: Option<String>,
    pub note: Option<String>,
    pub text_format_runs: Option<Vec<TextFormatRun>>,
    pub data_validation: Option<DataValidationRule>,
    pub pivot_table: Option<PivotTable>,
    pub data_source_table: Option<DataSourceTable>,
    pub data_source_formula: Option<DataSourceFormula>,
}
