use crate::domain::{Event, Participant};
use futures::stream::TryStreamExt;
use mongodb::{bson::{doc, oid::ObjectId, DateTime as BsonDateTime}, error::Error, results::{InsertOneResult, UpdateResult, DeleteResult}, Database};
use chrono::Utc;

const COLLECTION_NAME: &str = "events";

pub async fn create(db: &Database, event: &Event) -> Result<InsertOneResult, Error> {
    db.collection::<Event>(COLLECTION_NAME).insert_one(event, None).await
}

pub async fn find_all(db: &Database) -> Result<Vec<Event>, Error> {
    let mut cursor = db.collection::<Event>(COLLECTION_NAME).find(doc! {}, None).await?;
    let mut events = Vec::new();
    while let Some(event) = cursor.try_next().await? {
        events.push(event);
    }
    Ok(events)
}

pub async fn find_by_location_id(db: &Database, location_id: &ObjectId) -> Result<Vec<Event>, Error> {
    let filter = doc! { "location_id": location_id };
    let mut cursor = db.collection::<Event>(COLLECTION_NAME).find(filter, None).await?;
    let mut events = Vec::new();
    while let Some(event) = cursor.try_next().await? {
        events.push(event);
    }
    Ok(events)
}

pub async fn find_by_id(db: &Database, id: &ObjectId) -> Result<Option<Event>, Error> {
    db.collection::<Event>(COLLECTION_NAME).find_one(doc! { "_id": id }, None).await
}

pub async fn find_upcoming(db: &Database, limit: i64) -> Result<Vec<Event>, Error> {
    let now = Utc::now();
    let bson_now = BsonDateTime::from_millis(now.timestamp_millis());
    let filter = doc! { 
        "datetime": { "$gte": bson_now },
        "confirmed": true 
    };
    let options = mongodb::options::FindOptions::builder()
        .sort(doc! { "datetime": 1 })
        .limit(limit)
        .build();
    
    let mut cursor = db.collection::<Event>(COLLECTION_NAME).find(filter, options).await?;
    let mut events = Vec::new();
    while let Some(event) = cursor.try_next().await? {
        events.push(event);
    }
    Ok(events)
}

pub async fn update(db: &Database, id: &ObjectId, event: &Event) -> Result<UpdateResult, Error> {
    let filter = doc! { "_id": id };
    let event_doc = mongodb::bson::to_document(event)?;
    let update = doc! { "$set": event_doc };
    db.collection::<Event>(COLLECTION_NAME).update_one(filter, update, None).await
}

pub async fn add_participant(db: &Database, event_id: &ObjectId, participant: &Participant) -> Result<(), Error> {
    let filter = doc! { "_id": event_id };
    let update = doc! {
        "$push": {
            "participants": mongodb::bson::to_bson(participant)?
        }
    };
    db.collection::<Event>(COLLECTION_NAME).update_one(filter, update, None).await?;
    Ok(())
}

pub async fn update_participant_status(
    db: &Database, 
    event_id: &ObjectId, 
    user_id: &ObjectId, 
    status: &str,
    payment: &str
) -> Result<(), Error> {
    let filter = doc! { 
        "_id": event_id,
        "participants.user_id": user_id
    };
    let update = doc! {
        "$set": {
            "participants.$.status": status,
            "participants.$.payment": payment
        }
    };
    db.collection::<Event>(COLLECTION_NAME).update_one(filter, update, None).await?;
    Ok(())
}

pub async fn remove_participant(db: &Database, event_id: &ObjectId, user_id: &ObjectId) -> Result<(), Error> {
    let filter = doc! { "_id": event_id };
    let update = doc! {
        "$pull": {
            "participants": { "user_id": user_id }
        }
    };
    db.collection::<Event>(COLLECTION_NAME).update_one(filter, update, None).await?;
    Ok(())
}

pub async fn delete(db: &Database, id: &ObjectId) -> Result<DeleteResult, Error> {
    db.collection::<Event>(COLLECTION_NAME).delete_one(doc! { "_id": id }, None).await
} 