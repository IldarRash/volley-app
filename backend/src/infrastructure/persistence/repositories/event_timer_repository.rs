use crate::domain::EventTimer;
use futures::stream::TryStreamExt;
use mongodb::{bson::{doc, oid::ObjectId}, error::Error, results::{InsertOneResult, UpdateResult, DeleteResult}, Database};

const COLLECTION_NAME: &str = "event_timers";

pub async fn create(db: &Database, timer: &EventTimer) -> Result<InsertOneResult, Error> {
    db.collection::<EventTimer>(COLLECTION_NAME).insert_one(timer, None).await
}

pub async fn find_all(db: &Database) -> Result<Vec<EventTimer>, Error> {
    let mut cursor = db.collection::<EventTimer>(COLLECTION_NAME).find(doc! {}, None).await?;
    let mut timers = Vec::new();
    while let Some(timer) = cursor.try_next().await? {
        timers.push(timer);
    }
    Ok(timers)
}

pub async fn find_active(db: &Database) -> Result<Vec<EventTimer>, Error> {
    let filter = doc! { "active": true };
    let mut cursor = db.collection::<EventTimer>(COLLECTION_NAME).find(filter, None).await?;
    let mut timers = Vec::new();
    while let Some(timer) = cursor.try_next().await? {
        timers.push(timer);
    }
    Ok(timers)
}

pub async fn find_by_id(db: &Database, id: &ObjectId) -> Result<Option<EventTimer>, Error> {
    db.collection::<EventTimer>(COLLECTION_NAME).find_one(doc! { "_id": id }, None).await
}

pub async fn update(db: &Database, id: &ObjectId, timer: &EventTimer) -> Result<UpdateResult, Error> {
    let filter = doc! { "_id": id };
    let timer_doc = mongodb::bson::to_document(timer)?;
    let update = doc! { "$set": timer_doc };
    db.collection::<EventTimer>(COLLECTION_NAME).update_one(filter, update, None).await
}

pub async fn toggle_active(db: &Database, id: &ObjectId, active: bool) -> Result<(), Error> {
    let filter = doc! { "_id": id };
    let update = doc! { "$set": { "active": active } };
    db.collection::<EventTimer>(COLLECTION_NAME).update_one(filter, update, None).await?;
    Ok(())
}

pub async fn delete(db: &Database, id: &ObjectId) -> Result<DeleteResult, Error> {
    db.collection::<EventTimer>(COLLECTION_NAME).delete_one(doc! { "_id": id }, None).await
} 