use rocket::{get, post, serde::json::Json, http::Status};
use crate::domain::{Event, Participant, ParticipantStatus, PaymentStatus};
use crate::infrastructure::persistence::repositories::event_repository;
use crate::presentation::middleware::auth::AuthUser;
use rocket_db_pools::Connection;
use crate::AppDatabase;
use uuid::Uuid;

#[get("/locations/<location_id>/events")]
pub async fn get_events_by_location(mut db: Connection<AppDatabase>, location_id: &str) -> Result<Json<Vec<Event>>, Status> {
    let uuid = Uuid::parse_str(location_id)
        .map_err(|_| Status::BadRequest)?;
    
    let events = event_repository::find_by_location_id(&mut db, &uuid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Json(events))
}

#[get("/events/upcoming?<limit>")]
pub async fn get_upcoming_events(mut db: Connection<AppDatabase>, limit: Option<i64>) -> Result<Json<Vec<Event>>, Status> {
    let limit = limit.unwrap_or(10);
    
    let events = event_repository::find_upcoming(&mut db, limit)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Json(events))
}

#[post("/events/<event_id>/join")]
pub async fn join_event(mut db: Connection<AppDatabase>, event_id: &str, user: AuthUser) -> Result<Status, Status> {
    let event_uuid = Uuid::parse_str(event_id)
        .map_err(|_| Status::BadRequest)?;
    
    let user_uuid = Uuid::parse_str(&user.0.user_id)
        .map_err(|_| Status::BadRequest)?;
    
    // Check if event exists and is not full
    let mut event = event_repository::find_by_id(&mut db, &event_uuid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;

    if event.is_full() {
        return Err(Status::Conflict);
    }

    // Prevent duplicate registrations
    if event.participants.iter().any(|p| p.user_id == user_uuid) {
        return Err(Status::Conflict);
    }

    // Add participant
    let participant = Participant {
        user_id: user_uuid,
        status: ParticipantStatus::Pending,
        payment: PaymentStatus::Unpaid,
    };

    event_repository::add_participant(&mut db, &event_uuid, &participant)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Status::Ok)
}

#[post("/events/<event_id>/leave")]
pub async fn leave_event(mut db: Connection<AppDatabase>, event_id: &str, user: AuthUser) -> Result<Status, Status> {
    let event_uuid = Uuid::parse_str(event_id)
        .map_err(|_| Status::BadRequest)?;
    
    let user_uuid = Uuid::parse_str(&user.0.user_id)
        .map_err(|_| Status::BadRequest)?;
    
    event_repository::remove_participant(&mut db, &event_uuid, &user_uuid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Status::Ok)
} 