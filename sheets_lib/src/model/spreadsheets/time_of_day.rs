use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TimeOfDay {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub nanos: u32,
}
