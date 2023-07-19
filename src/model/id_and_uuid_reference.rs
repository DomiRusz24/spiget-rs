
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdAndUuidReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
impl std::fmt::Display for IdAndUuidReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}