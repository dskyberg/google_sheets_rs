use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataExecutionErrorCodeType {
    DataExecutionErrorCodeUnspecified,
    TimedOut,
    TooManyRows,
    TooManyColumns,
    TooManyCells,
    Engine,
    ParameterInvalid,
    UnsupportedDataType,
    DuplicateColumnNames,
    Interrupted,
    ConcurrentQuery,
    Other,
    TooManyCharsPerCell,
    DataNotFound,
    PermissionDenied,
    MissingColumnAlias,
    ObjectNotFound,
    ObjectInErrorState,
    ObjectSpecInvalid,
    DataExecutionCancelled,
}
