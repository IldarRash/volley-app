// backend/src/api/beosand.rs

use rocket::serde::json::Json;
use crate::models::{location::Location, event::Event};
use crate::persistence::{db::get_db, repository::{location_repository, event_repository}};

#[get("/locations")]
pub async fn get_locations() -> Json<Vec<Location>> {
    let db = get_db().await.unwrap();
    let locations = location_repository::find_all(&db).await;
    Json(locations.unwrap_or_default())
}

#[get("/locations/<location_id>/events")]
pub async fn get_events_by_location(location_id: &str) -> Json<Vec<Event>> {
    let db = get_db().await.unwrap();
    let events = event_repository::find_by_location_id(&db, location_id).await;
    Json(events.unwrap_or_default())
} 