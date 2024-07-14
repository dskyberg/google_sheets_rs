use serde::Serialize;

use crate::SheetProperties;

use super::request_type::RequestType;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    requests: Vec<RequestType>,
    include_spreadsheet_in_response: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_ranges: Option<Vec<String>>,
    response_include_grid_data: bool,
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
}
