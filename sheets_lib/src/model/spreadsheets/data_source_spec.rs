use serde::{Deserialize, Serialize};

use super::{BigQueryDataSourceSpec, DataSourceParameter};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSourceSpec {
    pub parameters: Vec<DataSourceParameter>,
    // Union field spec can be only one of the following:
    pub big_query: BigQueryDataSourceSpec, // End of list of possible types for union field spec.
}
