
use serde::{Serialize, Deserialize};
use super::InlineResponse200StatusFetchPage;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse200StatusFetch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<InlineResponse200StatusFetchPage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
impl std::fmt::Display for InlineResponse200StatusFetch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}