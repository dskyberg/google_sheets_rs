use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryQuerySpec {
    pub raw_query: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryTableSpec {
    pub table_project_id: Option<String>,
    pub table_id: String,
    pub dataset_id: String,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigQueryDataSourceSpec {
    pub project_id: String,
    // Union field spec can be only one of the following:
    pub query_spec: Option<BigQueryQuerySpec>,
    pub table_spec: Option<BigQueryTableSpec>, // End of list of possible types for union field spec.
}
