pub struct SpreadsheetQueryParams {
    pub include_grid_data: Option<bool>,
}

impl Default for SpreadsheetQueryParams {
    fn default() -> Self {
        Self::new()
    }
}

impl SpreadsheetQueryParams {
    pub fn new() -> Self {
        Self {
            include_grid_data: Some(true),
        }
    }
}

impl std::fmt::Display for SpreadsheetQueryParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::<String>::new();

        if let Some(igd) = &self.include_grid_data {
            params.push(format!("includeGridData={}", igd));
        }
        match params.is_empty() {
            true => write!(f, ""),
            false => write!(f, "?{}", params.join("&")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spreadsheet_query_params() {
        let params = SpreadsheetQueryParams::default();
        println!("{params}");
    }
}
