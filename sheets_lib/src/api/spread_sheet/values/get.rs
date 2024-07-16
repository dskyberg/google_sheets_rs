use anyhow::Result;

use crate::{http, ApiConfig};

use crate::{ValueRange, ValueRenderOption};

use super::ValueQueryParams;

/// Calls the Google Sheets api to get the values of a spreadshet.
/// This will get the values for the entire sheet.
pub async fn get(api_config: &ApiConfig, spreadsheet_id: &str, range: &str) -> Result<ValueRange> {
    let params = ValueQueryParams::new().with_value_render_option(ValueRenderOption::default());

    let url = format!(
        "{}/spreadsheets/{}/values/{}{}",
        &api_config.base_url, spreadsheet_id, range, params
    );
    let value_range = http::get::<ValueRange>(api_config, &url).await?;
    Ok(value_range)
}
