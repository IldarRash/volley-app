// backend/src/persistence/repository/location_repository.rs

use crate::models::location::Location;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, error::Error, results::InsertOneResult, Database};

pub async fn find_all(db: &Database) -> Result<Vec<Location>, Error> {
    let collection = db.collection::<Location>("locations");
    let mut cursor = collection.find(doc! {}, None).await?;
    let mut locations = Vec::new();
    while let Some(location) = cursor.try_next().await? {
        locations.push(location);
    }
    Ok(locations)
}

pub async fn create(db: &Database, location: &Location) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<Location>("locations");
    collection.insert_one(location, None).await
}

pub async fn clear(db: &Database) -> Result<(), Error> {
    let collection = db.collection::<Location>("locations");
    collection.delete_many(doc! {}, None).await?;
    Ok(())
} 