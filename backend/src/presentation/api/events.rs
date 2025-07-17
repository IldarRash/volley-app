use rocket::{get, post, serde::json::Json, http::Status};
use mongodb::bson::oid::ObjectId;
use crate::domain::{Event, Participant, ParticipantStatus, PaymentStatus};
use crate::infrastructure::persistence::{get_db, repositories::event_repository};
use crate::presentation::middleware::auth::AuthUser;

#[get("/locations/<location_id>/events")]
pub async fn get_events_by_location(location_id: &str) -> Result<Json<Vec<Event>>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    
    let oid = ObjectId::parse_str(location_id)
        .map_err(|_| Status::BadRequest)?;
    
    let events = event_repository::find_by_location_id(&db, &oid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Json(events))
}

#[get("/events/upcoming?<limit>")]
pub async fn get_upcoming_events(limit: Option<i64>) -> Result<Json<Vec<Event>>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let limit = limit.unwrap_or(10);
    
    let events = event_repository::find_upcoming(&db, limit)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Json(events))
}

#[post("/events/<event_id>/join")]
pub async fn join_event(event_id: &str, user: AuthUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    
    let event_oid = ObjectId::parse_str(event_id)
        .map_err(|_| Status::BadRequest)?;
    
    let user_oid = ObjectId::parse_str(&user.0.user_id)
        .map_err(|_| Status::BadRequest)?;
    
    // Check if event exists and is not full
    let event = event_repository::find_by_id(&db, &event_oid)
        .await
        .map_err(|_| Status::InternalServerError)?
        .ok_or(Status::NotFound)?;
    
    if event.is_full() {
        return Err(Status::Conflict);
    }
    
    // Add participant
    let participant = Participant {
        user_id: user_oid,
        status: ParticipantStatus::Pending,
        payment: PaymentStatus::Unpaid,
    };
    
    event_repository::add_participant(&db, &event_oid, &participant)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Status::Ok)
}

#[post("/events/<event_id>/leave")]
pub async fn leave_event(event_id: &str, user: AuthUser) -> Result<Status, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    
    let event_oid = ObjectId::parse_str(event_id)
        .map_err(|_| Status::BadRequest)?;
    
    let user_oid = ObjectId::parse_str(&user.0.user_id)
        .map_err(|_| Status::BadRequest)?;
    
    event_repository::remove_participant(&db, &event_oid, &user_oid)
        .await
        .map_err(|_| Status::InternalServerError)?;
    
    Ok(Status::Ok)
} 