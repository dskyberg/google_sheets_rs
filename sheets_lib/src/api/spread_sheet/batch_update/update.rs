use anyhow::Result;

use crate::{http, ApiConfig};

use super::{request::Request, response::Response};

// POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}:batchUpdate
pub async fn update<'a>(
    api_config: &ApiConfig,
    spreadsheet_id: &str,
    request: &Request,
) -> Result<Response> {
    let url = format!(
        "{}/spreadsheets/{spreadsheet_id}:batchUpdate",
        api_config.base_url
    );

    let result = http::post::<Request, Response>(api_config, &url, request).await?;
    Ok(result)
}
