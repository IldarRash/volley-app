// backend/src/models/user.rs

use crate::models::{Gender, PlayerPosition, PlayerProps};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in the system.
/// This struct is used for both database persistence and API communication.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub username: String,
    pub encoded_password: String,
    pub email: String,
    pub score: i32,
    pub positions: Vec<PlayerPosition>,
    pub player_props: Vec<PlayerProps>,
    pub gender: Gender,
    pub bio: Option<String>,
    pub image: Option<String>,
} 