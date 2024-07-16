#![allow(dead_code)]
use anyhow::Result;

use google_auth::fetch_access_token;
use sheets_lib::{
    api::{spread_sheet, ApiConfig},
    ValueRange,
};

const SPREADSHEET_ID: &str = "11zLQT1J_G2X-Vbgm4Wtm8LhJnXnuWeD5zzkkPp7fcEc";
const SHEET_NAME: &str = "Accounts";

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = fetch_access_token().await?;
    let api_config = ApiConfig::new(&access_token.value)
        .with_fields("sheets(properties,data(rowData.values(effectiveValue,formattedValue)))");

    let new_sheet_name = format!("{SHEET_NAME}_new");

    // Fetch a spreadhseet
    let spreadsheet = spread_sheet::get(&api_config, SPREADSHEET_ID).await?;
    println!("Fetched spreadsheet");
    let sheet = spreadsheet.get_sheet(SHEET_NAME)?;

    // Alternatively, fetch spreadsheet values (by A1 range)
    // TODO: Fix me!
    let _values = spread_sheet::values::get(&api_config, SPREADSHEET_ID, SHEET_NAME).await?;
    println!("Fetched values");

    // Add a new sheet to a spreadsheet
    let request = spread_sheet::batch_update::Request::add_sheet(&new_sheet_name);
    let _response =
        spread_sheet::batch_update::update(&api_config, SPREADSHEET_ID, &request).await?;
    println!("Batch Update added sheet");

    // Update sheet values
    let mut value_range = ValueRange::from(sheet);
    // Set the A1 range to the entire sheet
    value_range.range = new_sheet_name.clone();
    let _response =
        spread_sheet::values::update(&api_config, &value_range, SPREADSHEET_ID, &new_sheet_name)
            .await?;
    println!("Updated Values");

    // Clear the last 2 rows of the new sheet
    let range = format!("{}!21:1000", &new_sheet_name);
    let _response = spread_sheet::values::clear(&api_config, SPREADSHEET_ID, &range).await?;
    println!("Cleared Values");
    Ok(())
}
