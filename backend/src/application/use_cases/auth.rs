use crate::domain::{User, validators};
use crate::infrastructure::persistence::repositories::user_repository;
use mongodb::Database;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // username
    pub user_id: String, // ObjectId as string
    pub role: String,
    pub exp: usize,
}

const SECRET_KEY: &[u8] = b"your_super_secret_key"; // TODO: Move to config

pub async fn register_user(
    db: &Database,
    username: &str,
    password: &str,
) -> Result<User, String> {
    // Validate input
    validators::validate_username(username).map_err(|e| e.to_string())?;
    validators::validate_password(password).map_err(|e| e.to_string())?;

    // Check if user already exists
    if let Ok(Some(_)) = user_repository::find_by_username(db, username).await {
        return Err("Username already exists".to_string());
    }

    // Hash password
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|_| "Failed to hash password")?;

    // Create new user
    let mut user = User::new(username.to_string(), password_hash);

    // Save to database
    let insert_result = user_repository::create(db, &user)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;

    // Set the generated ID
    if let Some(inserted_id) = insert_result.inserted_id.as_object_id() {
        user.id = Some(inserted_id);
    }

    // Return user (password_hash won't be serialized)
    Ok(user)
}

pub async fn login_user(
    db: &Database,
    username: &str,
    password: &str,
) -> Result<(User, String), String> {
    // Find user
    let user = user_repository::find_by_username(db, username)
        .await
        .map_err(|_| "Database error")?
        .ok_or("Invalid username or password")?;

    // Verify password
    if !verify(password, &user.password_hash).unwrap_or(false) {
        return Err("Invalid username or password".to_string());
    }

    // Generate JWT token
    let user_id = user.id.as_ref()
        .ok_or("User ID not found")?
        .to_string();

    let claims = Claims {
        sub: user.username.clone(),
        user_id,
        role: serde_json::to_string(&user.role).unwrap().trim_matches('"').to_string(),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
        .map_err(|_| "Failed to generate token")?;

    Ok((user, token))
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| "Invalid token".to_string())
} 