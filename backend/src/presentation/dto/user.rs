use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub telegram_id: Option<String>,
    pub instagram_link: Option<String>,
    pub image_url: Option<String>,
} 