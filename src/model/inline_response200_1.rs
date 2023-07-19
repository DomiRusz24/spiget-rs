
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse2001 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}
impl std::fmt::Display for InlineResponse2001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}