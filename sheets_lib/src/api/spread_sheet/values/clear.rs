use anyhow::Result;
use serde::Deserialize;

use crate::{http, ApiConfig};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearValuesResponse {
    pub spreadsheet_id: String,
    cleared_range: String,
}

/// Calls the Google Sheets api to clear the values of a spreadshet within the given range.
pub async fn clear(
    api_config: &ApiConfig,
    spreadsheet_id: &str,
    range: &str,
) -> Result<ClearValuesResponse> {
    let url = format!(
        "{}/spreadsheets/{}/values/{}:clear",
        &api_config.base_url, spreadsheet_id, range
    );
    let response = http::post_empty::<ClearValuesResponse>(api_config, &url).await?;
    Ok(response)
}
