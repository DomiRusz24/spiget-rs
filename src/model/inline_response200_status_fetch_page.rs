
use serde::{Serialize, Deserialize};
use super::InlineResponse200StatusFetchPageItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InlineResponse200StatusFetchPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<InlineResponse200StatusFetchPageItem>,
}
impl std::fmt::Display for InlineResponse200StatusFetchPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}