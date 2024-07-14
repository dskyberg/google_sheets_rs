#![allow(dead_code)]
use anyhow::Result;

use google_auth::fetch_access_token;
use sheets_lib::{
    api::{spread_sheet, Api},
    Sheet, Spreadsheet, ValueRange,
};

const SPREADSHEET_ID: &str = "11zLQT1J_G2X-Vbgm4Wtm8LhJnXnuWeD5zzkkPp7fcEc";
const SHEET_NAME: &str = "Accounts";

async fn get_spreadsheet(api: &Api, spreadsheet_id: &str) -> Result<Spreadsheet> {
    let spreadsheet = spread_sheet::get(api, spreadsheet_id).await?;
    Ok(spreadsheet)
}

async fn get_sheet(api: &Api, spreadsheet_id: &str, sheet_name: &str) -> Result<ValueRange> {
    let sheet = spread_sheet::values::get(api, spreadsheet_id, sheet_name).await?;
    Ok(sheet)
}

async fn add_sheet(
    api: &Api,
    spreadsheet_id: &str,
    sheet_name: &str,
) -> Result<spread_sheet::batch_update::Response> {
    // Generate the request model object
    let request = spread_sheet::batch_update::Request::add_sheet(sheet_name);

    // Make the batch update
    let response = spread_sheet::batch_update::update(api, spreadsheet_id, &request).await?;

    Ok(response)
}

async fn update_sheet(
    api: &Api,
    sheet: &Sheet,
    spreadsheet_id: &str,
    sheet_name: &str,
) -> Result<()> {
    let mut value_range = ValueRange::from(sheet);

    value_range.range = sheet_name.to_string();

    spread_sheet::values::update(api, &value_range, spreadsheet_id, sheet_name).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = fetch_access_token().await?;
    let new_sheet_name = format!("{SHEET_NAME}_new");

    let api = Api::new(&access_token.value)
        .with_fields("sheets(properties,data(rowData.values(effectiveValue,formattedValue)))");

    let spreadsheet = get_spreadsheet(&api, SPREADSHEET_ID).await?;
    let sheet = spreadsheet.get_sheet(SHEET_NAME)?;

    //let values = ValueRange::from(sheet);
    //let json = serde_json::to_string_pretty(&values).unwrap();
    //println!("{json}");

    let _response = add_sheet(&api, SPREADSHEET_ID, &new_sheet_name).await?;
    update_sheet(&api, sheet, SPREADSHEET_ID, &new_sheet_name).await?;

    Ok(())
}
