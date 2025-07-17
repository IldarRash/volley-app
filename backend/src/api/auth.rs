use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::persistence::{db::get_db, repository::user_repository};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

const SECRET_KEY: &[u8] = b"your_super_secret_key"; // Replace with a real secret key

#[post("/register", data = "<auth_request>")]
pub async fn register(auth_request: Json<AuthRequest>) -> Result<Json<User>, Status> {
    let db = get_db().await.unwrap();
    let hashed_password = hash(&auth_request.password, DEFAULT_COST).map_err(|_| Status::InternalServerError)?;

    let new_user = User {
        id: None,
        username: auth_request.username.clone(),
        password_hash: hashed_password,
        role: "user".to_string(),
        rating: 0.0,
        telegram_id: None,
        instagram_link: None,
        subscriptions: None,
    };

    match user_repository::create(&db, &new_user).await {
        Ok(_) => Ok(Json(new_user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<auth_request>")]
pub async fn login(auth_request: Json<AuthRequest>) -> Result<Json<AuthResponse>, Status> {
    let db = get_db().await.unwrap();
    
    let user = user_repository::find_by_username(&db, &auth_request.username)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or_else(|| Status::Unauthorized)?;

    if verify(&auth_request.password, &user.password_hash).unwrap_or(false) {
        let claims = Claims {
            sub: user.username.clone(),
            role: user.role.clone(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
            .map_err(|_| Status::InternalServerError)?;
        
        Ok(Json(AuthResponse { token }))
    } else {
        Err(Status::Unauthorized)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthUser(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                if let Ok(token_data) = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &Validation::default()) {
                    return Outcome::Success(AuthUser(token_data.claims));
                }
            }
        }
        Outcome::Forward(())
    }
} 