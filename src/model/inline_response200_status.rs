
use serde::{Serialize, Deserialize};
use super::InlineResponse200StatusFetch;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse200Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch: Option<InlineResponse200StatusFetch>,
}
impl std::fmt::Display for InlineResponse200Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}