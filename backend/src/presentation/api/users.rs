use rocket::{get, put, serde::json::Json, http::Status};
use crate::domain::User;
use crate::infrastructure::persistence::repositories::user_repository;
use crate::presentation::middleware::auth::AuthUser;
use crate::presentation::dto::user::UpdateUserRequest;
use rocket_db_pools::Connection;
use crate::AppDatabase;
use uuid::Uuid;

#[get("/me")]
pub async fn get_me(mut db: Connection<AppDatabase>, user: AuthUser) -> Result<Json<User>, Status> {
    let uuid = Uuid::parse_str(&user.0.user_id).map_err(|_| Status::BadRequest)?;
    
    let user = user_repository::find_by_id(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    Ok(Json(user))
}

#[put("/me", data = "<request>")]
pub async fn update_me(mut db: Connection<AppDatabase>, user: AuthUser, request: Json<UpdateUserRequest>) -> Result<Json<User>, Status> {
    let uuid = Uuid::parse_str(&user.0.user_id).map_err(|_| Status::BadRequest)?;
    
    let mut current_user = user_repository::find_by_id(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    if let Some(telegram_id) = &request.telegram_id {
        current_user.telegram_id = Some(telegram_id.clone());
    }
    
    if let Some(instagram_link) = &request.instagram_link {
        current_user.instagram_link = Some(instagram_link.clone());
    }

    if let Some(image_url) = &request.image_url {
        current_user.image_url = Some(image_url.clone());
    }
    
    let updated_user = user_repository::update(&mut db, &uuid, &current_user)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Json(updated_user))
} 