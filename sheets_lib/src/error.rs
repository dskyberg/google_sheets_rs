use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JSON Object expected")]
    ExpectObject,
    #[error("JSON array expected")]
    ExpectArray,
    #[error("JSON String expected")]
    ExpectString,
    #[error("Get Row error: {0}")]
    GetRow(&'static str),
    #[error("Unrecognized header: {0}")]
    UnrecognizedHeader(String),
    #[error("Field not found: {0}")]
    FieldNotFound(String),
    #[error("Expected headers but none found")]
    ExpectedHeaders,
    #[error("column index must be between 1 and 18,278")]
    A1ColIndex,
    #[error("Malformed A1 address: {0}")]
    A1Address(String),
    #[error("Malformed R1C1 address: {0}")]
    R1C1Address(String),
    #[error("Bad CellRange {0}")]
    BadCellRange(String),
    #[error("Bad Range {0}")]
    BadRange(String),
    #[error("From anyhow")]
    FromAnyhow(#[from] anyhow::Error),
}
