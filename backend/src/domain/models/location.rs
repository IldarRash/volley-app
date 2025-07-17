use serde::{Serialize, Deserialize};
use sqlx::types::uuid::Uuid;

/// Represents geographical coordinates
#[derive(Debug, Serialize, Deserialize, Clone, Default, sqlx::FromRow)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

/// Represents a geographical location
#[derive(Debug, Serialize, Deserialize, Clone, Default, sqlx::FromRow)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
    #[sqlx(json)]
    pub coordinates: Coordinates,
    pub confirmed: bool,
    pub image_url: Option<String>,
}

impl Location {
    pub fn new(name: String, coordinates: Coordinates) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            coordinates,
            confirmed: false,
            image_url: None,
        }
    }
} 