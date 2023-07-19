
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse2003 {
    #[serde(rename = "failedConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_connections: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<f64>,
}
impl std::fmt::Display for InlineResponse2003 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}