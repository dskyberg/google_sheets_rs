use serde::{Deserialize, Serialize};

use crate::Spreadsheet;

use super::response_type::ResponseType;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    spreadsheet_id: String,
    replies: Vec<ResponseType>,
    updated_spreadsheet: Option<Spreadsheet>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
