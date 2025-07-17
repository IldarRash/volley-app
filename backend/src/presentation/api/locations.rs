use rocket::{get, serde::json::Json, http::Status};
use crate::domain::Location;
use crate::infrastructure::persistence::{get_db, repositories::location_repository};

#[get("/")]
pub async fn get_locations() -> Result<Json<Vec<Location>>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let locations = location_repository::find_all_confirmed(&db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(locations))
}

#[get("/all")]
pub async fn get_all_locations() -> Result<Json<Vec<Location>>, Status> {
    let db = get_db().await.map_err(|_| Status::InternalServerError)?;
    let locations = location_repository::find_all(&db)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(locations))
} 