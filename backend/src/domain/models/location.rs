use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

/// Represents a geographical location
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub coordinates: [f64; 2],
    pub confirmed: bool,
    pub image_url: Option<String>,
}

impl Location {
    pub fn new(name: String, coordinates: [f64; 2]) -> Self {
        Self {
            id: None,
            name,
            coordinates,
            confirmed: false,
            image_url: None,
        }
    }
} 