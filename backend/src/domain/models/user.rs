use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
pub use super::{UserRole, subscription::Subscription};

/// Domain model for User
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[serde(skip_serializing, default)]
    pub password_hash: String,
    pub role: UserRole,
    pub rating: f64,
    pub telegram_id: Option<String>,
    pub instagram_link: Option<String>,
    pub image_url: Option<String>,
    #[serde(default)]
    pub subscriptions: Vec<Subscription>,
    pub subscribed: bool,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: None,
            username,
            password_hash,
            role: UserRole::User,
            rating: 0.0,
            telegram_id: None,
            instagram_link: None,
            image_url: None,
            subscriptions: Vec::new(),
            subscribed: false,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
} 