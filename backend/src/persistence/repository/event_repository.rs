use crate::models::event::Event;
use futures::stream::TryStreamExt;
use mongodb::{bson::{doc, Bson}, error::Error, results::InsertOneResult, Database};
use uuid::Uuid;

const COLLECTION_NAME: &str = "events";

pub async fn get_by_location(db: &Database, location_id: Uuid) -> Result<Vec<Event>, Error> {
    let collection = db.collection::<Event>("events");
    let mut cursor = collection.find(doc! { "location_id": Bson::String(location_id.to_string()) }, None).await?;
    let mut events = Vec::new();
    while let Some(event) = cursor.try_next().await? {
        events.push(event);
    }
    Ok(events)
}

pub async fn create(db: &Database, event: &Event) -> Result<InsertOneResult, Error> {
    let collection = db.collection::<Event>("events");
    collection.insert_one(event, None).await
}

pub async fn find_all(db: &Database) -> Result<Vec<Event>, mongodb::error::Error> {
    let collection = db.collection::<Event>(COLLECTION_NAME);
    let mut cursor = collection.find(doc! {}, None).await?;
    let mut events = Vec::new();
    while let Some(event) = cursor.try_next().await? {
        events.push(event);
    }
    Ok(events)
}

pub async fn find_by_location_id(db: &Database, location_id: &str) -> Result<Vec<Event>, mongodb::error::Error> {
    let collection = db.collection::<Event>(COLLECTION_NAME);
    let oid = match mongodb::bson::oid::ObjectId::parse_str(location_id) {
        Ok(oid) => oid,
        Err(_) => return Ok(vec![]), // Return empty list if ID is invalid
    };
    let filter = doc! { "location_id": oid };
    let mut cursor = collection.find(filter, None).await?;
    let mut events = Vec::new();
    while let Some(event) = cursor.try_next().await? {
        events.push(event);
    }
    Ok(events)
} 