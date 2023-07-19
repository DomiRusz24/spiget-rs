
use serde::{Serialize, Deserialize};
use super::ResourceRating;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<ResourceRating>,
    #[serde(rename = "releaseDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
impl std::fmt::Display for ResourceVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}