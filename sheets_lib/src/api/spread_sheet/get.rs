use anyhow::Result;

use crate::{http, ApiConfig, Spreadsheet};

use super::spreadsheet_query_params::SpreadsheetQueryParams;

/// Calls the Google Sheets api to get an entire spreadshet.
/// This will get the values for the entire sheet.
pub async fn get(api_config: &ApiConfig, spreadsheet_id: &str) -> Result<Spreadsheet> {
    let params = SpreadsheetQueryParams::default();
    let mut url = format!(
        "{}/spreadsheets/{spreadsheet_id}{params}",
        api_config.base_url
    );

    if let Some(fields) = api_config.fields.as_ref() {
        url += &format!("&fields={}", *fields);
    }

    let spreadsheet = http::get::<Spreadsheet>(api_config, &url).await?;
    Ok(spreadsheet)
}
