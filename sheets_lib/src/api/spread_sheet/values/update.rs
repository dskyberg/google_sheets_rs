use anyhow::Result;
use serde::Deserialize;

use crate::{http, ApiConfig};

use crate::{ValueInputOption, ValueRange};

use super::ValueQueryParams;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateValuesResponse {
    pub spreadsheet_id: String,
    pub updated_range: String,
    pub updated_rows: u32,
    pub updated_columns: u32,
    pub updated_cells: u32,
    pub updated_data: Option<ValueRange>,
}

//PUT https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}
pub async fn update(
    api_config: &ApiConfig,
    value_range: &ValueRange,
    spreadsheet_id: &str,
    range: &str,
) -> Result<UpdateValuesResponse> {
    let params = ValueQueryParams::new()
        .with_value_input_option(ValueInputOption::default())
        .with_include_values_in_response(false);

    let url = format!(
        "{}/spreadsheets/{spreadsheet_id}/values/{range}{params}",
        &api_config.base_url,
    );

    http::put::<ValueRange, UpdateValuesResponse>(api_config, &url, value_range).await
}
