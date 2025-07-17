use rocket::{post, serde::json::Json, http::Status};
use crate::application::use_cases::auth::{register_user, login_user};
use crate::presentation::dto::auth::{RegisterRequest, LoginRequest, AuthResponse, UserInfo};
use crate::AppDatabase;
use rocket_db_pools::Connection;
use tracing::{info, warn};

#[post("/register", data = "<request>")]
pub async fn register(mut db: Connection<AppDatabase>, request: Json<RegisterRequest>) -> Result<Json<AuthResponse>, Status> {
    let _user = match register_user(&mut db, &request.username, &request.password).await {
        Ok(user) => user,
        Err(_) => return Err(Status::Conflict),
    };

    let (user, token) = match login_user(&mut db, &request.username, &request.password).await {
        Ok(result) => result,
        Err(_) => return Err(Status::InternalServerError),
    };
    
    let user_info = UserInfo {
        id: user.id.to_string(),
        username: user.username,
        role: serde_json::to_string(&user.role).unwrap().trim_matches('"').to_string(),
        rating: user.rating,
    };
    
    Ok(Json(AuthResponse {
        token,
        user: user_info,
    }))
}

#[post("/login", data = "<request>")]
pub async fn login(mut db: Connection<AppDatabase>, request: Json<LoginRequest>) -> Result<Json<AuthResponse>, Status> {
    match login_user(&mut db, &request.username, &request.password).await {
        Ok((user, token)) => {
            info!(username = %user.username, "User logged in successfully");
            let user_info = UserInfo {
                id: user.id.to_string(),
                username: user.username,
                role: serde_json::to_string(&user.role).unwrap().trim_matches('"').to_string(),
                rating: user.rating,
            };
            Ok(Json(AuthResponse {
                token,
                user: user_info,
            }))
        },
        Err(_) => {
            warn!(username = %request.username, "Failed login attempt");
            Err(Status::Unauthorized)
        }
    }
} 