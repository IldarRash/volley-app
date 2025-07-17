use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub telegram_id: Option<String>,
    pub instagram_link: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Deserialize)]
pub struct UserImportRecord {
    pub username: String,
    pub password: Option<String>,
    pub telegram_id: Option<i64>,
    pub instagram_link: Option<String>,
    pub rating: f64,
    pub role: Option<UserRole>,
} 