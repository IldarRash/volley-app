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