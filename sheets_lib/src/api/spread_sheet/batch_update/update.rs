use anyhow::Result;

use crate::{http, Api};

use super::{request::Request, response::Response};

// POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}:batchUpdate
pub async fn update<'a>(api: &Api, spreadsheet_id: &str, request: &Request) -> Result<Response> {
    let url = format!("{}/spreadsheets/{spreadsheet_id}:batchUpdate", api.base_url);

    let result = http::post::<Request, Response>(&url, &api.access_token, request).await?;
    Ok(result)
}
