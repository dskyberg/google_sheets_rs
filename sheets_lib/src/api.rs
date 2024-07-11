use anyhow::Result;

use crate::{
    batch_update_api,
    http::{get, post, put},
    SheetInfo, Spreadsheet, ValueInputOption, ValueRange,
};

/// Simple container to hold API specific stuff.
pub struct Api<'a> {
    /// Bse url of the Google Sheets APIs.
    base_url: &'a str,
    access_token: &'a str,
}

impl<'a> Api<'a> {
    pub fn new(access_token: &'a str) -> Self {
        Self {
            base_url: "https://sheets.googleapis.com/v4",
            access_token,
        }
    }

    fn format_url(&self, spreadsheet_id: &str, range: &str) -> String {
        format!(
            "{}/spreadsheets/{spreadsheet_id}/values/{range}",
            self.base_url
        )
    }

    /// Calls the Google Sheets api to get an entire spreadshet.
    /// This will get the values for the entire sheet.
    pub async fn get_spreadsheet(&self, spreadsheet_id: &str) -> Result<Spreadsheet> {
        let url = format!(
            "{}/spreadsheets/{spreadsheet_id}?includeGridData=true&fields=sheets.data(rowData.values(effectiveValue,formattedValue))",
            self.base_url
        );
        let spreadsheet = get::<Spreadsheet>(&url, self.access_token).await?;
        Ok(spreadsheet)
    }

    /// Calls the Google Sheets api to get the values of a spreadshet.
    /// This will get the values for the entire sheet.
    pub async fn get_spreadsheet_values(
        &self,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<SheetInfo> {
        let url = self.format_url(spreadsheet_id, range);
        let value_range = get::<ValueRange>(&url, self.access_token).await?;
        println!(
            "Range: {}, Major Dimension: {:?}",
            &value_range.range, &value_range.major_dimension
        );
        let sheet = SheetInfo::new().from_value_range(value_range)?;
        Ok(sheet)
    }
    //PUT https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}/values/{range}
    pub async fn spreadsheet_values_update(
        &self,
        value_range: &ValueRange,
        spreadsheet_id: &str,
        range: &str,
    ) -> Result<()> {
        let value_input_option = ValueInputOption::InputValueOptionUnspecified;

        let url = format!(
            "{}/spreadsheets/{spreadsheet_id}/values/{range}?{}&valueInputOption=Raw",
            self.base_url,
            value_input_option.to_query()
        );

        put::<ValueRange>(&url, self.access_token, value_range).await?;

        Ok(())
    }

    // POST https://sheets.googleapis.com/v4/spreadsheets/{spreadsheetId}:batchUpdate
    pub async fn spreadsheet_batch_update(
        &self,
        spreadsheet_id: &str,
        request: &batch_update_api::Request,
    ) -> Result<batch_update_api::Response> {
        let url = format!(
            "{}/spreadsheets/{spreadsheet_id}:batchUpdate",
            self.base_url
        );

        let result = post::<batch_update_api::Request, batch_update_api::Response>(
            &url,
            self.access_token,
            request,
        )
        .await?;
        println!("{:?}", result);
        Ok(result)
    }
}
