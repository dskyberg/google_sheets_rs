use serde::{Deserialize, Serialize};

use crate::SheetProperties;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    AddSheet { properties: SheetProperties },
}
