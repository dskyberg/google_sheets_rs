use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Editors {
    pub users: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    pub domain_users_can_edit: Option<bool>,
}
