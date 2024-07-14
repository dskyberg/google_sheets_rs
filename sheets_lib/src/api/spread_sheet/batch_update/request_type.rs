use serde::{Deserialize, Serialize};

use crate::SheetProperties;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestType {
    AddSheet { properties: SheetProperties },
}
