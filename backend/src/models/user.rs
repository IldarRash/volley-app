// backend/src/models/user.rs

use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub rating: f64,
    pub telegram_id: Option<String>,
    pub instagram_link: Option<String>,
} 