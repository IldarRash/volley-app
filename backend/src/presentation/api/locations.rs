use rocket::{get, serde::json::Json, http::Status};
use crate::domain::Location;
use crate::infrastructure::persistence::repositories::location_repository;
use rocket_db_pools::Connection;
use crate::AppDatabase;

#[get("/")]
pub async fn get_locations(mut db: Connection<AppDatabase>) -> Result<Json<Vec<Location>>, Status> {
    let locations = location_repository::find_all_confirmed(&mut db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(locations))
}

#[get("/all")]
pub async fn get_all_locations(mut db: Connection<AppDatabase>) -> Result<Json<Vec<Location>>, Status> {
    let locations = location_repository::find_all(&mut db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(locations))
} 