
use serde::{Serialize, Deserialize};
use super::{Icon, IdAndUuidReference, IdReference, ResourceFile, ResourceRating};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<IdReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "donationLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donation_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<ResourceFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<ResourceRating>,
    #[serde(rename = "releaseDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Vec<IdReference>>,
    #[serde(rename = "sourceCodeLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "testedVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tested_versions: Option<Vec<String>>,
    #[serde(rename = "updateDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<IdReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<IdAndUuidReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<IdReference>>,
}
impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}