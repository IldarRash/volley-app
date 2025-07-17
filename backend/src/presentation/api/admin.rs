use rocket::{get, post, put, delete, serde::json::Json, http::Status};
use mongodb::bson::oid::ObjectId;
use crate::domain::{Location, Event, EventTimer};
use crate::domain::models::UserRole;
use crate::infrastructure::persistence::{get_db, repositories::{location_repository, event_repository, event_timer_repository, user_repository}};
use crate::infrastructure::external::telegram;
use crate::presentation::middleware::auth::AdminUser;

// User management
#[get("/users")]
pub async fn get_users(_admin: AdminUser) -> Result<Json<Vec<crate::domain::User>>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let users = user_repository::find_all(&db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(users))
}

#[put("/users/<id>/role", data = "<role>")]
pub async fn set_user_role(id: &str, role: Json<UserRole>, _admin: AdminUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    
    let mut user = user_repository::find_by_id(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    user.role = role.into_inner();
    
    user_repository::update(&db, &oid, &user)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Status::Ok)
}

// Location management
#[post("/locations", data = "<location>")]
pub async fn create_location(location: Json<Location>, _admin: AdminUser) -> Result<Json<Location>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    location_repository::create(&db, &location)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(location)
}

#[put("/locations/<id>", data = "<location>")]
pub async fn update_location(id: &str, location: Json<Location>, _admin: AdminUser) -> Result<Json<Location>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    location_repository::update(&db, &oid, &location)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(location)
}

#[delete("/locations/<id>")]
pub async fn delete_location(id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    location_repository::delete(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::Ok)
}

#[put("/locations/<id>/confirm")]
pub async fn confirm_location(id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    
    let mut location = location_repository::find_by_id(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    location.confirmed = true;
    
    location_repository::update(&db, &oid, &location)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Status::Ok)
}

// Event management
#[post("/events", data = "<event>")]
pub async fn create_event(event: Json<Event>, _admin: AdminUser) -> Result<Json<Event>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    event_repository::create(&db, &event)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    telegram::notify_new_event(&event).await;
    
    Ok(event)
}

#[put("/events/<id>", data = "<event>")]
pub async fn update_event(id: &str, event: Json<Event>, _admin: AdminUser) -> Result<Json<Event>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    event_repository::update(&db, &oid, &event)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(event)
}

#[delete("/events/<id>")]
pub async fn delete_event(id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    event_repository::delete(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::Ok)
}

#[put("/events/<id>/confirm")]
pub async fn confirm_event(id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    
    let mut event = event_repository::find_by_id(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    event.confirmed = true;
    
    event_repository::update(&db, &oid, &event)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Status::Ok)
}

// Event Timer management
#[post("/event_timers", data = "<timer>")]
pub async fn create_event_timer(timer: Json<EventTimer>, _admin: AdminUser) -> Result<Json<EventTimer>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    event_timer_repository::create(&db, &timer)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(timer)
}

#[put("/event_timers/<id>", data = "<timer>")]
pub async fn update_event_timer(id: &str, timer: Json<EventTimer>, _admin: AdminUser) -> Result<Json<EventTimer>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    event_timer_repository::update(&db, &oid, &timer)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(timer)
}

#[delete("/event_timers/<id>")]
pub async fn delete_event_timer(id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let oid = ObjectId::parse_str(id).map_err(|_| Status::BadRequest)?;
    event_timer_repository::delete(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::Ok)
} 