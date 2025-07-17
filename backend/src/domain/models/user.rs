use serde::{Serialize, Deserialize};
use sqlx::types::uuid::Uuid;
pub use super::{subscription::Subscription, UserRole};

/// Domain model for User
#[derive(Debug, Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing, default)]
    pub password_hash: String,
    #[sqlx(try_from = "String")]
    pub role: UserRole,
    pub rating: f64,
    pub telegram_id: Option<String>,
    pub instagram_link: Option<String>,
    pub image_url: Option<String>,
    #[sqlx(json)]
    #[serde(default)]
    pub subscriptions: Vec<Subscription>,
    pub subscribed: bool,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
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