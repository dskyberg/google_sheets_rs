/// Request and response models for the batch_update API.
use serde::{Deserialize, Serialize};

use super::{SheetProperties, Spreadsheet};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestType {
    AddSheet { properties: SheetProperties },
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    requests: Vec<RequestType>,
    include_spreadsheet_in_response: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_ranges: Option<Vec<String>>,
    response_include_grid_data: bool,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    AddSheet { properties: SheetProperties },
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    spreadsheet_id: String,
    replies: Vec<ResponseType>,
    updated_spreadsheet: Option<Spreadsheet>,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
            include_spreadsheet_in_response: false,
            response_ranges: None,
            response_include_grid_data: false,
        }
    }
    pub fn with_request(mut self, request: RequestType) -> Self {
        self.requests.push(request);
        self
    }

    pub fn add_sheet(sheet_name: &str) -> Self {
        Self::new().with_request(RequestType::AddSheet {
            properties: (SheetProperties::new().with_title(sheet_name)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        let request = Request::new().with_request(RequestType::AddSheet {
            properties: (SheetProperties::new().with_title("My Title")),
        });
        let json = serde_json::to_string_pretty(&request).expect("oops");
        println!("{}", json);
    }

    #[test]
    fn test_response() {
        let json = r#"{
  "spreadsheetId": "11zLQT1J_G2X-Vbgm4Wtm8LhJnXnuWeD5zzkkPp7fcEc",
  "replies": [
    {
      "addSheet": {
        "properties": {
          "sheetId": 1569467798,
          "title": "My Title",
          "index": 1,
          "sheetType": "GRID",
          "gridProperties": {
            "rowCount": 1000,
            "columnCount": 26
          }
        }
      }
    }
  ]
}"#;
        let result = serde_json::from_str::<Response>(json).expect("deserialize oops");
        let result_json = serde_json::to_string_pretty(&result).expect("serialize oops");
        assert_eq!(json, result_json);
    }
}
