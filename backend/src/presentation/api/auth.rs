use rocket::{post, serde::json::Json, http::Status};
use crate::presentation::dto::auth::{RegisterRequest, LoginRequest, AuthResponse, UserInfo};
use crate::application::use_cases::auth::{register_user, login_user};
use crate::Db;
use rocket_db_pools::Connection;

#[post("/register", data = "<request>")]
pub async fn register(request: Json<RegisterRequest>, db: Connection<Db>) -> Result<Json<AuthResponse>, Status> {
    println!("Register request received: {:?}", request);
    let db = &db.database("volleyApp");
    
    let user = match register_user(db, &request.username, &request.password).await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Registration error: {}", e);
            return Err(Status::BadRequest);
        }
    };
    
    // Auto-login after registration
    let (user, token) = match login_user(db, &request.username, &request.password).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Auto-login error: {}", e);
            return Err(Status::InternalServerError);
        }
    };
    
    let user_info = UserInfo {
        id: user.id.unwrap().to_string(),
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
pub async fn login(request: Json<LoginRequest>, db: Connection<Db>) -> Result<Json<AuthResponse>, Status> {
    println!("Login request received: {:?}", request);
    let db = &db.database("volleyApp");
    
    let (user, token) = match login_user(db, &request.username, &request.password).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Login error: {}", e);
            return Err(Status::Unauthorized);
        }
    };
    
    let user_info = UserInfo {
        id: user.id.unwrap().to_string(),
        username: user.username,
        role: serde_json::to_string(&user.role).unwrap().trim_matches('"').to_string(),
        rating: user.rating,
    };
    
    Ok(Json(AuthResponse {
        token,
        user: user_info,
    }))
} 