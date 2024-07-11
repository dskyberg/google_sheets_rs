use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelativeDateType {
    RelativeDateNotSpecified,
    PastYear,
    PastMonth,
    PastWeek,
    Yesterday,
    Today,
    Tomorrow,
}
