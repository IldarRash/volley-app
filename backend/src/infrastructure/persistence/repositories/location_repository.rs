use crate::domain::Location;
use mongodb::{bson::{doc, oid::ObjectId, to_bson}, error::Error, results::{InsertOneResult, UpdateResult, DeleteResult}, Database};
use futures::stream::TryStreamExt;

const COLLECTION_NAME: &str = "locations";

pub async fn create(db: &Database, location: &Location) -> Result<InsertOneResult, Error> {
    db.collection::<Location>(COLLECTION_NAME).insert_one(location, None).await
}

pub async fn find_by_id(db: &Database, id: &ObjectId) -> Result<Option<Location>, Error> {
    db.collection::<Location>(COLLECTION_NAME).find_one(doc! { "_id": id }, None).await
}

pub async fn find_all_confirmed(db: &Database) -> Result<Vec<Location>, Error> {
    let mut cursor = db.collection::<Location>(COLLECTION_NAME).find(doc! { "confirmed": true }, None).await?;
    let mut locations = Vec::new();
    while let Some(location) = cursor.try_next().await? {
        locations.push(location);
    }
    Ok(locations)
}

pub async fn find_all(db: &Database) -> Result<Vec<Location>, Error> {
    let mut cursor = db.collection::<Location>(COLLECTION_NAME).find(doc! {}, None).await?;
    let mut locations = Vec::new();
    while let Some(location) = cursor.try_next().await? {
        locations.push(location);
    }
    Ok(locations)
}

pub async fn update(db: &Database, id: &ObjectId, location: &Location) -> Result<UpdateResult, Error> {
    let filter = doc! { "_id": id };
    let update = doc! {
        "$set": {
            "name": &location.name,
            "coordinates": to_bson(&location.coordinates).unwrap(),
            "confirmed": location.confirmed,
            "image_url": &location.image_url,
        }
    };
    db.collection::<Location>(COLLECTION_NAME).update_one(filter, update, None).await
}

pub async fn delete(db: &Database, id: &ObjectId) -> Result<DeleteResult, Error> {
    db.collection::<Location>(COLLECTION_NAME).delete_one(doc! { "_id": id }, None).await
}

pub async fn clear(db: &Database) -> Result<(), Error> {
    db.collection::<Location>(COLLECTION_NAME).delete_many(doc! {}, None).await?;
    Ok(())
} 