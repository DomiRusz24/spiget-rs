
use serde::{Serialize, Deserialize};
use super::{InlineResponse200Stats, InlineResponse200Status};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse200 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<InlineResponse200Stats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InlineResponse200Status>,
}
impl std::fmt::Display for InlineResponse200 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}