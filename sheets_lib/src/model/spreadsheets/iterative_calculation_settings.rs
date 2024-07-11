use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IterativeCalculationSettings {
    pub max_iterations: u32,
    pub convergence_threshold: f32,
}
