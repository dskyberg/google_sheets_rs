use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;

use crate::Error;
use crate::{ConvertValue, DimensionType, GetField, ValueRange};

#[derive(Debug, PartialEq)]
pub struct SheetInfo {
    /// Sheet headers.  If there is a header row, it will come from that.
    /// Note: Headers must be unique, else header lookups will fail.
    headers: BTreeMap<String, usize>,
    has_header_row: bool,
    pub values: Vec<Vec<String>>,
}

impl Default for SheetInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SheetInfo {
    pub fn new() -> Self {
        Self {
            headers: BTreeMap::new(),
            has_header_row: true,
            values: vec![],
        }
    }

    // Generate a A1 style range for the sheet

    /// Private method to add a header to the header tree
    fn add_header(&mut self, header: &str) {
        let x = self.headers.iter().position(|s| s.0 == header);
        if x.is_none() {
            let idx = self.headers.len();
            self.headers.insert(header.to_string(), idx);
        }
    }

    #[inline(always)]
    pub fn header_idx(&self, header: &str) -> Option<usize> {
        self.headers.get(header).copied()
    }

    pub fn header_at(&self, idx: usize) -> Option<String> {
        self.headers
            .iter()
            .filter(|(_, value)| **value == idx)
            .collect::<Vec<(&String, &usize)>>()
            .first()
            .map(|(key, _)| key.to_string())
    }

    pub fn get_row_col_idx(&self, row_idx: usize, col_idx: usize) -> Result<Option<String>> {
        if row_idx >= self.values.len() {
            return Ok(None);
        }
        let row = self
            .values
            .get(row_idx)
            .ok_or(Error::GetRow("row not found"))?;
        if col_idx >= row.len() {
            // This isn't an error.  Google Sheets may have truncated the row due to empty colums
            return Ok(None);
        }

        // Since we know there are enough columns, if get failes, it's an error
        Ok(Some(
            row.get(col_idx)
                .ok_or(Error::GetRow("col value fetch error"))
                .map(|s| s.to_owned())?,
        ))
    }

    pub fn get_at(&self, row_idx: usize, col_name: &str) -> Result<Option<String>> {
        let col_idx = self
            .header_idx(col_name)
            .ok_or(Error::UnrecognizedHeader(col_name.to_string()))?;
        self.get_row_col_idx(row_idx, col_idx)
    }

    pub fn from_value_range(mut self, json_sheet: ValueRange) -> Result<Self> {
        let mut row_idx = 0;
        if self.has_header_row {
            if json_sheet.values.is_empty() {
                // Expected headers, but there aren't any.
                return Err(Error::ExpectedHeaders.into());
            }
            let row = json_sheet
                .values
                .first()
                .ok_or(Error::GetRow("No row returned for headers"))?;
            row.iter().for_each(|header| self.add_header(header));
            row_idx += 1;
        }
        // Now just consume all the rows
        self.values = json_sheet.values[row_idx..].to_vec();
        Ok(self)
    }

    pub fn from_json(mut self, json: &Value) -> Result<Self> {
        let json_sheet = json.to_obj()?;
        // Get the values field from the sheet...
        let values = json_sheet.get_field("values")?;
        // ... and read as an array
        let value_vec = values.to_array()?;

        if value_vec.is_empty() {
            return Ok(self);
        }
        let mut row_idx = 0;
        // Grab the headers
        if self.has_header_row {
            let headers = value_vec[row_idx].to_array()?;
            for header in headers {
                self.add_header(header.to_str()?);
            }
            row_idx += 1;
        }

        // Now process the rows
        while row_idx < value_vec.len() {
            // Convert each row to Vec<String>
            let value_vec = value_vec[row_idx].to_array()?;
            let mut row: Vec<String> = Vec::new();
            for value in value_vec {
                row.push(value.to_str()?.to_string());
            }
            /*
            let row = value_vec
                .iter()
                .map(|value| value.to_str())
                .collect::<Vec<Result<&str>>>()
                .into_iter()
                .collect::<Result<Vec<&str>>>()?
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            */
            row_idx += 1;
            self.values.push(row);
        }

        Ok(self)
    }
    //------------------- Build Methods ---------------------
    pub fn without_header_row(mut self) -> Self {
        self.has_header_row = false;
        self
    }

    /// For sheets that don't have a header row, add headers.
    /// Note: If a header already exists, it will not be re-added.
    pub fn with_header(mut self, header: &str) -> Self {
        self.add_header(header);
        self
    }
}

impl From<&SheetInfo> for ValueRange {
    fn from(sheet: &SheetInfo) -> Self {
        let major_dimension = DimensionType::Rows;

        ValueRange {
            range: "Accounts_new".to_string(),
            major_dimension,
            values: sheet.values.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    fn build_sheet() -> SheetInfo {
        SheetInfo::new()
            .without_header_row()
            .with_header("Account Name")
            .with_header("Account Id")
            .with_header("Account Owner")
            .with_header("Yes No")
            .with_header("Int Val")
            .with_header("Currenty")
            .with_header("Num Val")
    }
    #[test]
    fn test_with_header() {
        let sheet = build_sheet();
        assert_eq!(sheet.header_idx("Account Name"), Some(0));
        assert_eq!(sheet.header_at(1), Some("Account Id".to_string()))
    }

    #[test]
    fn test_from_json() {
        let input = r#"{
    "values": [
        [
            "Account Name",
            "Account Id",
            "Account Owner",
            "Yes No",
            "Int Val",
            "Currenty",
            "Num Val"
        ],
        ["A Company", "ebe88151-ab75-48ab-804b-db224283675a", "Bob Smith", "No", "1", "$10.00", "0.1"]
    ]
} "#;
        let json = serde_json::from_str::<Value>(input).expect("Failed to parse json");
        let sheet = SheetInfo::new().from_json(&json).expect("wtf?");

        let val = sheet.get_at(0, "Account Name");
        assert!(val.is_ok());
        assert_eq!(val.unwrap(), Some("A Company".to_string()));
    }
}
