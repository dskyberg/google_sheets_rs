#![allow(dead_code)]
use anyhow::Result;

use google_auth::fetch_access_token;

use sheets_lib::{
    batch_update_api::Request as BatchUpdateRequest,
    batch_update_api::Response as BatchUpdateResponse, *,
};

const SPREADSHEET_ID: &str = "11zLQT1J_G2X-Vbgm4Wtm8LhJnXnuWeD5zzkkPp7fcEc";
const SHEET_NAME: &str = "Accounts";

async fn get_spreadsheet<'a>(api: &Api<'a>, spreadsheet_id: &str) -> Result<Spreadsheet> {
    let spreadsheet = api.get_spreadsheet(spreadsheet_id).await?;
    println!("Successfully retrieved Spreadsheet");
    println!("{:#?}", &spreadsheet);
    Ok(spreadsheet)
}

async fn get_sheet<'a>(api: &Api<'a>, spreadsheet_id: &str, sheet_name: &str) -> Result<SheetInfo> {
    let sheet = api
        .get_spreadsheet_values(spreadsheet_id, sheet_name)
        .await?;
    println!("Successfully retrieved sheet values");
    println!("{:#?}", &sheet);
    Ok(sheet)
}

async fn add_sheet<'a>(
    api: &Api<'a>,
    spreadsheet_id: &str,
    sheet_name: &str,
) -> Result<BatchUpdateResponse> {
    // Generate the request model object
    let request = BatchUpdateRequest::add_sheet(sheet_name);

    // Make the batch update
    let response = api
        .spreadsheet_batch_update(spreadsheet_id, &request)
        .await?;

    println!("{:#?}", &response);
    Ok(response)
}

async fn update_sheet<'a>(
    api: &Api<'a>,
    sheet: &SheetInfo,
    spreadsheet_id: &str,
    sheet_name: &str,
) -> Result<()> {
    let value_range = ValueRange::from(sheet);

    api.spreadsheet_values_update(&value_range, spreadsheet_id, &format!("{}_new", sheet_name))
        .await?;
    println!("Successfully updated sheet");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = fetch_access_token().await?;

    let api = Api::new(&access_token.value);

    //let _sheet = get_sheet(&api, SPREADSHEET_ID, SHEET_NAME).await?;
    let _json = get_spreadsheet(&api, SPREADSHEET_ID).await?;
    Ok(())
}
