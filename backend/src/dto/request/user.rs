// backend/src/dto/request/user.rs

use crate::models::{Gender, PlayerPosition};
use serde::Deserialize;

/// Represents the request payload for user registration.
#[derive(Debug, Deserialize)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub gender: Gender,
    pub positions: Vec<PlayerPosition>,
}

#[derive(Debug, Deserialize)]
pub struct UserImportRecord {
    pub username: String,
    pub email: String,
    pub telegram_id: String,
    pub rating: f64,
    pub instagram_link: String,
} 