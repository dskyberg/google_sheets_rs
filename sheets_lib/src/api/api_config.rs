use reqwest::header::HeaderMap;

/// Simple container to hold API specific stuff.
pub struct ApiConfig {
    /// Bse url of the Google Sheets APIs.
    pub base_url: String,
    pub access_token: String,
    pub fields: Option<String>,
    pub headers: HeaderMap,
}

impl ApiConfig {
    pub fn new(access_token: &str) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            format!("Bearer {access_token}").parse().unwrap(),
        );
        headers.insert("Accept", "application/json".parse().unwrap());

        Self {
            base_url: "https://sheets.googleapis.com/v4".to_owned(),
            access_token: access_token.to_owned(),
            fields: None, //sheets(properties,data(rowData.values(effectiveValue,formattedValue)))
            headers,
        }
    }

    pub fn with_fields(mut self, fields: &str) -> Self {
        self.fields = Some(fields.to_owned());
        self
    }
}
