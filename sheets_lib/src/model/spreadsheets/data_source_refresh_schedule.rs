use serde::{Deserialize, Serialize};

use super::TimeOfDay;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSourceRefreshScopeType {
    DataSourceRefreshScopeUnspecified,
    AllDataSources,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeekType {
    DayOfWeekUnspecified,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interval {
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshDailySchedule {
    pub start_time: TimeOfDay,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshWeeklySchedule {
    pub start_time: TimeOfDay,
    pub days_of_week: Vec<DayOfWeekType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshMonthlySchedule {
    pub start_time: TimeOfDay,
    pub days_of_month: Vec<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceRefreshSchedule {
    pub enabled: bool,
    pub refresh_scope: DataSourceRefreshScopeType,
    pub next_run: Interval,

    // Union field schedule_config can be only one of the following:
    pub daily_schedule: DataSourceRefreshDailySchedule,
    pub weekly_schedule: DataSourceRefreshWeeklySchedule,
    pub monthly_schedule: DataSourceRefreshMonthlySchedule,
    // End of list of possible types for union field schedule_config.
}
