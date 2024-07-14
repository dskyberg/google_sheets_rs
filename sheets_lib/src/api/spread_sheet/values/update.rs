use anyhow::Result;

use crate::{http, Api};

use crate::ValueRange;

use super::ValueQueryParams;

//PUT https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}
pub async fn update(
    api: &Api,
    value_range: &ValueRange,
    spreadsheet_id: &str,
    range: &str,
) -> Result<()> {
    let params = ValueQueryParams::default();
    let url = format!(
        "{}/spreadsheets/{spreadsheet_id}/values/{range}{params}",
        &api.base_url,
    );

    http::put::<ValueRange>(&url, &api.access_token, value_range).await?;

    Ok(())
}
