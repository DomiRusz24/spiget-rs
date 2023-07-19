
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse200Stats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_updates: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_versions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<f64>,
}
impl std::fmt::Display for InlineResponse200Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}