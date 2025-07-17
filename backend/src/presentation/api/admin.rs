use rocket::{get, post, put, delete, serde::json::Json, http::Status, Data};
use crate::domain::{Location, Event, EventTimer, User, UserRole};
use crate::infrastructure::persistence::repositories::{location_repository, event_repository, event_timer_repository, user_repository};
use crate::presentation::middleware::auth::AdminUser;
use crate::application::use_cases::admin::AdminUseCases;
use rocket::data::{ToByteUnit};
use rocket_db_pools::Connection;
use crate::AppDatabase;
use uuid::Uuid;

// User management
#[post("/users/import/csv", data = "<csv_data>")]
pub async fn import_users_csv(
    mut db: Connection<AppDatabase>,
    csv_data: Data<'_>,
    _admin: AdminUser,
) -> Result<Json<String>, Status> {
    let bytes = csv_data
        .open(2.mebibytes())
        .into_bytes()
        .await
        .map_err(|_| Status::BadRequest)?
        .into_inner();

    match AdminUseCases::import_users_from_csv(&mut db, &bytes).await {
        Ok(count) => Ok(Json(format!("Successfully imported {} users.", count))),
        Err(e) => {
            eprintln!("Failed to import users: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/users/import/json", data = "<json_data>")]
pub async fn import_users_json(
    mut db: Connection<AppDatabase>,
    json_data: Data<'_>,
    _admin: AdminUser,
) -> Result<Json<String>, Status> {
    let bytes = json_data
        .open(2.mebibytes())
        .into_bytes()
        .await
        .map_err(|_| Status::BadRequest)?
        .into_inner();

    match AdminUseCases::import_users_from_json(&mut db, &bytes).await {
        Ok(count) => Ok(Json(format!("Successfully imported {} users.", count))),
        Err(e) => {
            eprintln!("Failed to import users: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/users")]
pub async fn get_users(mut db: Connection<AppDatabase>, _admin: AdminUser) -> Result<Json<Vec<User>>, Status> {
    let users = user_repository::find_all(&mut db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(users))
}

#[put("/users/<id>/role", data = "<role>")]
pub async fn set_user_role(mut db: Connection<AppDatabase>, id: &str, role: Json<UserRole>, _admin: AdminUser) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    
    let mut user = user_repository::find_by_id(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    user.role = role.into_inner();
    
    user_repository::update(&mut db, &uuid, &user)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Status::Ok)
}

// Location management
#[post("/locations", data = "<location>")]
pub async fn create_location(mut db: Connection<AppDatabase>, location: Json<Location>, _admin: AdminUser) -> Result<Json<Location>, Status> {
    let new_location = location_repository::create(&mut db, &location.into_inner())
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(new_location))
}

#[put("/locations/<id>", data = "<location>")]
pub async fn update_location(mut db: Connection<AppDatabase>, id: &str, location: Json<Location>, _admin: AdminUser) -> Result<Json<Location>, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    let updated_location = location_repository::update(&mut db, &uuid, &location.into_inner())
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(updated_location))
}

#[delete("/locations/<id>")]
pub async fn delete_location(mut db: Connection<AppDatabase>, id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    location_repository::delete(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::Ok)
}

#[put("/locations/<id>/confirm")]
pub async fn confirm_location(mut db: Connection<AppDatabase>, id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    
    let mut location = location_repository::find_by_id(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    location.confirmed = true;
    
    location_repository::update(&mut db, &uuid, &location)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Status::Ok)
}

// Event management
#[post("/events", data = "<event>")]
pub async fn create_event(mut db: Connection<AppDatabase>, event: Json<Event>, _admin: AdminUser) -> Result<Json<Event>, Status> {
    let new_event = event_repository::create(&mut db, &event.into_inner())
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(new_event))
}

#[put("/events/<id>", data = "<event>")]
pub async fn update_event(mut db: Connection<AppDatabase>, id: &str, event: Json<Event>, _admin: AdminUser) -> Result<Json<Event>, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    let updated_event = event_repository::update(&mut db, &uuid, &event.into_inner())
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(updated_event))
}

#[delete("/events/<id>")]
pub async fn delete_event(mut db: Connection<AppDatabase>, id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    event_repository::delete(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::Ok)
}

#[put("/events/<id>/confirm")]
pub async fn confirm_event(mut db: Connection<AppDatabase>, id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    
    let mut event = event_repository::find_by_id(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
        
    event.confirmed = true;
    
    event_repository::update(&mut db, &uuid, &event)
        .await
        .map_err(|_| Status::InternalServerError)?;
        
    Ok(Status::Ok)
}

// Event Timer management
#[post("/event_timers", data = "<timer>")]
pub async fn create_event_timer(mut db: Connection<AppDatabase>, timer: Json<EventTimer>, _admin: AdminUser) -> Result<Json<EventTimer>, Status> {
    let new_timer = event_timer_repository::create(&mut db, &timer.into_inner())
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(new_timer))
}

#[put("/event_timers/<id>", data = "<timer>")]
pub async fn update_event_timer(mut db: Connection<AppDatabase>, id: &str, timer: Json<EventTimer>, _admin: AdminUser) -> Result<Json<EventTimer>, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    let updated_timer = event_timer_repository::update(&mut db, &uuid, &timer.into_inner())
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(updated_timer))
}

#[delete("/event_timers/<id>")]
pub async fn delete_event_timer(mut db: Connection<AppDatabase>, id: &str, _admin: AdminUser) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(id).map_err(|_| Status::BadRequest)?;
    event_timer_repository::delete(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Status::Ok)
} 