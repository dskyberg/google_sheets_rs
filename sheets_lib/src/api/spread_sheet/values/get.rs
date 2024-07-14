use anyhow::Result;

use crate::{http, Api};

use crate::ValueRange;

use super::ValueQueryParams;

/// Calls the Google Sheets api to get the values of a spreadshet.
/// This will get the values for the entire sheet.
pub async fn get(api: &Api, spreadsheet_id: &str, range: &str) -> Result<ValueRange> {
    let params = ValueQueryParams::default();

    let url = format!(
        "{}/spreadsheets/{}/values/{}{}",
        &api.base_url, spreadsheet_id, range, params
    );
    let value_range = http::get::<ValueRange>(&url, &api.access_token).await?;
    Ok(value_range)
}
