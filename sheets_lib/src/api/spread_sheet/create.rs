use anyhow::Result;

use crate::{http, ApiConfig, Spreadsheet};

/// Calls the Google Sheets api to create a  spreadshet.
/// The resulting spreadhseet is returned.
pub async fn create(api_config: &ApiConfig, spreadsheet: &Spreadsheet) -> Result<Spreadsheet> {
    let url = format!("{}/spreadsheets", api_config.base_url);

    let spreadsheet = http::post::<Spreadsheet, Spreadsheet>(api_config, &url, spreadsheet).await?;
    Ok(spreadsheet)
}
