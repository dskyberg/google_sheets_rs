pub mod spread_sheet;

/// Simple container to hold API specific stuff.
pub struct Api {
    /// Bse url of the Google Sheets APIs.
    base_url: String,
    access_token: String,
    fields: Option<String>,
}

impl Api {
    pub fn new(access_token: &str) -> Self {
        Self {
            base_url: "https://sheets.googleapis.com/v4".to_owned(),
            access_token: access_token.to_owned(),
            fields: None, //sheets(properties,data(rowData.values(effectiveValue,formattedValue)))
        }
    }

    pub fn with_fields(mut self, fields: &str) -> Self {
        self.fields = Some(fields.to_owned());
        self
    }
}
